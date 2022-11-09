use chrono::prelude::*;
use std::{
    collections::HashSet,
    ops::{Deref, Not},
    sync::Arc,
};

use personal_finance::{
    account::{Category, Name, Number},
    balance::Balance,
};

use crate::{
    error::{AccountError, LedgerError, TransactionError},
    events::EventPointer,
    Event,
};

/// A ledger id is a string starting with any alphanumeric character [a-zA-Z0-9]
/// followed by any valid character in [a-zA-Z0-9_-]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct LedgerId(String);

impl LedgerId {
    pub fn new(id: &str) -> Option<Self> {
        id.starts_with(|x: char| x.is_ascii_alphanumeric())
            .then_some(())
            .and_then(|_| {
                id.chars()
                    .skip(1)
                    .all(|ch| ch.is_ascii_alphanumeric() || matches!(ch, '_' | '-'))
                    .then_some(LedgerId(id.to_owned()))
            })
    }
}

/// LedgerResolver keeps a tally on all available ledgers in the system
#[derive(Debug, PartialEq, Eq, Default)]
pub struct LedgerResolver {
    ledgers: HashSet<LedgerId>,
    history: Vec<Event>,
}

impl LedgerResolver {
    pub fn new(events: &[Event]) -> Self {
        let mut ledgers = HashSet::new();

        for event in events {
            match event {
                Event::LedgerCreated { id } => {
                    ledgers.insert(id.clone());
                }
                _ => (),
            }
        }

        Self {
            ledgers,
            history: events.to_vec(),
        }
    }

    pub fn create(&mut self, id: LedgerId) -> Result<&[Event], LedgerError> {
        self.ledgers
            .contains(&id)
            .not()
            .then(|| {
                self.ledgers.insert(id.clone());
                self.history.push(Event::LedgerCreated { id });
                &self.history[self.history.len() - 1..]
            })
            .ok_or(LedgerError::AlreadyExists)
    }

    pub fn get<T: AsRef<str>>(&self, id: T) -> Option<LedgerId> {
        todo!()
    }
}

#[derive(Debug)]
pub struct Ledger {
    id: LedgerId,
    chart: HashSet<Number>,
    history: Vec<EventPointer>,
}

impl Ledger {
    pub fn new(id: LedgerId, events: &[EventPointer]) -> Option<Self> {
        events.iter().position(
            |x| matches!(x.deref(), Event::LedgerCreated { id: ledger_id } if *ledger_id == id ),
        )
        .map(|index| {
            let chart = Default::default();
            let history = events.to_vec();

            let mut ledger = Ledger { id, chart, history };

            ledger.apply(&events[index..]);
            ledger
        })
    }

    pub fn open_account(
        &mut self,
        number: Number,
        name: Name,
        category: Category,
    ) -> Result<&[EventPointer], AccountError> {
        self.chart
            .contains(&number)
            .not()
            .then_some(())
            .ok_or(AccountError::Opened(number.number()))
            .map(|_| {
                vec![Arc::new(Event::AccountOpened {
                    ledger: self.id.clone(),
                    id: number,
                    name,
                    category,
                })]
            })
            .map(|issued_events| self.apply_new_events(issued_events))
    }

    pub fn close_account(&mut self, id: Number) -> Result<&[EventPointer], AccountError> {
        self.chart
            .contains(&id)
            .then(|| {
                vec![Arc::new(Event::AccountClosed {
                    ledger: self.id.clone(),
                    account: id,
                })]
            })
            .ok_or(AccountError::NotExist)
            .map(|issued_events| self.apply_new_events(issued_events))
    }

    fn check_balance(&self, transactions: &[(Number, Balance)]) -> Result<(), TransactionError> {
        let mut account_exists = true;
        let mut balance_partition = (0u32, 0u32);
        for (number, amount) in transactions.iter() {
            account_exists = account_exists
                .then(|| self.chart.contains(&number))
                .unwrap_or_default();

            if !account_exists {
                break;
            }

            balance_partition = match *amount {
                Balance::Debit(x) => (
                    balance_partition
                        .0
                        .checked_add(x.amount())
                        .expect("Amount overflow"),
                    balance_partition.1,
                ),
                Balance::Credit(x) => (
                    balance_partition.0,
                    balance_partition
                        .1
                        .checked_add(x.amount())
                        .expect("Amount overflow"),
                ),
            }
        }

        let is_zero_balance = balance_partition.0 == balance_partition.1;
        match (account_exists, is_zero_balance) {
            (false, _) => Err(TransactionError::AccountDoesntExist),
            (_, false) => Err(TransactionError::ImbalancedTranasactions),
            _ => Ok(()),
        }
    }

    pub fn transaction<T: Into<String>>(
        &mut self,
        description: T,
        transactions: &[(Number, Balance)],
        date: Date<Utc>,
    ) -> Result<&[EventPointer], TransactionError> {
        transactions
            .len()
            .gt(&0)
            .then_some(())
            .ok_or(TransactionError::EmptyTransaction)
            .and_then(|()| self.check_balance(transactions))
            .map(|_| {
                vec![Arc::new(Event::Transaction {
                    ledger: self.id.clone(),
                    description: description.into(),
                    date,
                    transactions: transactions.to_vec(),
                })]
            })
            .map(|events| self.apply_new_events(events))
    }

    fn apply_new_events(&mut self, events: Vec<EventPointer>) -> &[EventPointer] {
        let number_of_new_events = events.len();
        self.apply(&events);
        self.history.extend(events);

        let index = self.history.len().saturating_sub(number_of_new_events);
        &self.history[index..]
    }

    fn apply(&mut self, events: &[EventPointer]) {
        for event in events {
            match event.deref() {
                Event::AccountOpened { ledger, id, .. } if *ledger == self.id => {
                    self.chart.insert(*id);
                }
                Event::AccountClosed { ledger, account } if *ledger == self.id => {
                    self.chart.remove(account);
                }
                Event::Transaction { ledger, .. } if *ledger == self.id => {}
                _ => {}
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use proptest::proptest;

    proptest! {
        #[test]
        fn invalid_ledger_ids(s in "[_-][a-zA-Z0-9]*") {
            assert_eq!(LedgerId::new(&s), None);
        }
    }

    proptest! {
        #[test]
        fn valid_ledger_ids(s in "[a-zA-Z0-9][a-zA-Z0-9_-]*") {
            assert_eq!(LedgerId::new(&s), Some(LedgerId(s)))
        }
    }
}
