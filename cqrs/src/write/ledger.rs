use std::{collections::HashSet, ops::Not};

use personal_finance::account::{Category, Name, Number};

use crate::{
    error::{AccountError, LedgerError},
    Event,
    Journal,
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
#[derive(Debug, PartialEq, Eq)]
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
}

pub struct Ledger {
    id: LedgerId,
    chart: HashSet<Number>,
    journal: Journal,
    history: Vec<Event>,
}

impl Ledger {
    pub fn new(id: LedgerId, events: &[Event]) -> Self {
        let chart = Default::default();
        let journal = Journal::default();
        let history = events.to_vec();

        let mut ledger = Ledger {
            id,
            chart,
            journal,
            history,
        };

        ledger.apply(&events);

        ledger
    }

    pub fn open_account(
        &mut self,
        number: Number,
        name: Name,
        category: Category,
    ) -> Result<&[Event], AccountError> {
        self.chart
            .contains(&number)
            .not()
            .then_some(())
            .ok_or(AccountError::Opened(number.number()))
            .map(|_| {
                vec![Event::AccountOpened {
                    ledger: self.id.clone(),
                    id: number,
                    name,
                    category,
                }]
            })
            .map(|issued_events| self.apply_new_events(issued_events))
    }

    fn apply_new_events(&mut self, events: Vec<Event>) -> &[Event] {
        let len = events.len();
        self.apply(&events);
        self.history.extend(events);

        let index = self.history.len().checked_sub(len).unwrap_or_default();
        &self.history[index..]
    }

    fn apply(&mut self, events: &[Event]) {
        for event in events {
            match event {
                Event::AccountOpened { ledger, id, .. } if *ledger == self.id => {
                    self.chart.insert(*id);
                }
                Event::AccountClosed { ledger, account } if *ledger == self.id => todo!(),
                Event::Transaction {
                    ledger,
                    account,
                    amount,
                    journal,
                } if *ledger == self.id => todo!(),
                Event::Journal {
                    ledger,
                    id,
                    description,
                    date,
                } if *ledger == self.id => todo!(),
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
