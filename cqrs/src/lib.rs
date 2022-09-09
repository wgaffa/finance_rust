use std::collections::HashSet;

use error::AccountError;
use events::Event;
use personal_finance::account::{Category, Name, Number};

pub mod behaviour;
pub mod error;
pub mod events;
pub mod identifier;
pub mod stream;

pub struct Chart {
    data: HashSet<events::AccountId>,
    history: Vec<Event>,
}

impl Chart {
    pub fn new(history: &[Event]) -> Self {
        let mut chart = Self {
            data: Default::default(),
            history: history.to_vec(),
        };

        chart.apply(&history);

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
            .then_some(())
            .map(|_| {
                let issued_events = vec![Event::AccountOpened {
                    id: number.number(),
                    name: name.into(),
                    category,
                }];

                self.apply(&issued_events);

                self.history.extend(issued_events);

                let index = self.history.len().checked_sub(1).unwrap_or_default();
                let events = &self.history[index..];

                events
            })
            .ok_or(AccountError::AccountAlreadyOpened(number.number()))
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

impl Default for Chart {
    fn default() -> Self {
        Self {
            data: Default::default(),
            history: Vec::new(),
        }
    }
}
