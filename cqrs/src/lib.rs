use std::{collections::HashSet, ops::Neg};

use error::{AccountError, JournalError};
use events::{Balance, Event};
use personal_finance::account::{Category, Name, Number};

pub mod behaviour;
pub mod error;
pub mod events;
pub mod identifier;
pub mod stream;

pub type JournalId = u32;
pub type AccountId = u32;

#[derive(Default)]
pub struct Chart {
    data: HashSet<AccountId>,
    history: Vec<Event>,
}

impl Chart {
    pub fn new(history: &[Event]) -> Self {
        let mut chart = Self {
            data: Default::default(),
            history: history.to_vec(),
        };

        chart.apply(history);

        chart
    }

    pub fn open(
        &mut self,
        number: Number,
        name: Name,
        category: Category,
    ) -> Result<&[Event], AccountError> {
        let account_doesnt_exist = !self.data.contains(&number.number());
        account_doesnt_exist
            .then(|| (number.number(), name.into_inner(), category))
            .map(|(id, name, category)| vec![Event::AccountOpened { id, name, category }])
            .map(|issued_events| {
                let len = issued_events.len();
                self.apply(&issued_events);
                self.history.extend(issued_events);

                len
            })
            .map(|len| {
                let index = self.history.len().checked_sub(len).unwrap_or_default();
                &self.history[index..]
            })
            .ok_or_else(|| AccountError::AccountAlreadyOpened(number.number()))
    }

    fn apply(&mut self, events: &[Event]) {
        for event in events {
            match event {
                Event::AccountOpened { id, .. } => {
                    self.data.insert(*id);
                }
                Event::AccountClosed(id) => {
                    self.data.remove(id);
                }
                _ => {}
            }
        }
    }
}

pub struct Journal {
    current_id: JournalId,
    accounts: HashSet<AccountId>,
    history: Vec<Event>,
}

impl Journal {
    pub fn new(history: &[Event]) -> Self {
        let mut journal = Self {
            current_id: 0,
            accounts: HashSet::new(),
            history: history.to_vec(),
        };

        journal.apply(history);

        journal
    }

    pub fn entry<T: Into<String>>(
        &mut self,
        description: T,
        transactions: &[(AccountId, Balance)],
    ) -> Result<&[Event], JournalError> {
        transactions
            .into_iter()
            .fold(
                0, |sum, (_, amount)| sum + match *amount {
                Balance::Credit(x) => i64::from(x).neg(),
                Balance::Debit(x) => i64::from(x),
            })
            .eq(&0)
            .then_some(())
            .ok_or(JournalError::ImbalancedTranasactions)
            .and_then(|()| {
                self.current_id
                    .checked_add(1)
                    .ok_or(JournalError::JournalLimitReached)
            })
            .map(|id| {
                let mut v = vec![Event::Journal {
                    id,
                    description: description.into(),
                }];
                v.extend(
                    transactions
                        .into_iter()
                        .map(|(account, amount)| Event::Transaction {
                            account: *account,
                            amount: *amount,
                            journal: id,
                        }),
                );

                v
            })
            .map(|events| {
                self.apply(&events);
                let len = self.history.len();
                self.history.extend(events);
                len
            })
            .map(|len| &self.history[len..])
    }

    fn apply(&mut self, events: &[Event]) {
        for event in events {
            match event {
                Event::AccountOpened { id, .. } => {
                    self.accounts.insert(*id);
                }
                Event::AccountClosed(id) => {
                    self.accounts.remove(id);
                }
                Event::Journal { id, .. } => self.current_id = self.current_id.max(*id),
                _ => {}
            }
        }
    }
}
