use std::{collections::HashSet, ops::Neg};

use chrono::prelude::*;
use personal_finance::account::Number;

use crate::{Balance, Event, JournalError, JournalId};

#[derive(Default)]
pub struct Journal {
    current_id: JournalId,
    accounts: HashSet<Number>,
    history: Vec<Event>,
}

fn transcribe_amount(amount: Balance) -> i64 {
    match amount {
        Balance::Debit(x) => i64::from(x.amount()),
        Balance::Credit(x) => i64::from(x.amount()).neg(),
    }
}

fn make_journal(
    id: JournalId,
    description: String,
    transactions: &[(Number, Balance)],
    date: Date<Utc>,
) -> Vec<Event> {
    let mut v = vec![Event::Journal {
        id,
        description,
        date,
    }];
    v.extend(
        transactions
            .iter()
            .map(|(account, amount)| Event::Transaction {
                account: *account,
                amount: *amount,
                journal: id,
            }),
    );

    v
}

fn next_id(current: JournalId) -> Result<JournalId, JournalError> {
    current
        .checked_add(1)
        .ok_or(JournalError::JournalLimitReached)
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
        transactions: &[(Number, Balance)],
        date: Date<Utc>,
    ) -> Result<&[Event], JournalError> {
        transactions
            .len()
            .gt(&0)
            .then_some(())
            .ok_or(JournalError::EmptyTransaction)
            .and_then(|()| {
                let mut account_exists = true;
                let mut balance = 0;
                for (number, amount) in transactions.iter() {
                    account_exists = account_exists
                        .then(|| self.accounts.contains(&number))
                        .unwrap_or_default();

                    if !account_exists {
                        break;
                    }

                    balance += transcribe_amount(*amount);
                }

                match (account_exists, balance) {
                    (false, _) => Err(JournalError::InvalidTransaction),
                    (_, sum) if sum != 0 => Err(JournalError::ImbalancedTranasactions),
                    _ => Ok(()),
                }
            })
            .and_then(|()| next_id(self.current_id))
            .map(|id| make_journal(id, description.into(), transactions, date))
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
