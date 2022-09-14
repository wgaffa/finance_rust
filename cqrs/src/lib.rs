use std::collections::HashSet;

use error::{AccountError, JournalError};
use events::Event;
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
        transactions: &[(AccountId, events::Balance)],
    ) -> Result<&[Event], JournalError> {
        let id = self
            .current_id
            .checked_add(1)
            .ok_or(JournalError::JournalLimitReached)?;

        let transactions = transactions.into_iter().map(|(account, amount)| Event::Transaction {
            account: *account,
            amount: *amount,
            journal: id,
        });

        let events = std::iter::once(Event::Journal { id, description: description.into() })
            .chain(transactions)
            .collect::<Vec<_>>();

        self.apply(&events);

        let len = self.history.len();
        self.history.extend(events);

        Ok(&self.history[len..])
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
