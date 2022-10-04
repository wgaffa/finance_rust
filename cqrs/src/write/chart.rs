use std::collections::HashSet;

use personal_finance::account::{Category, Name, Number};

use crate::{AccountError, AccountId, Event};

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
            .then_some(())
            .map(|()| {
                vec![Event::AccountOpened {
                    id: number,
                    name,
                    category,
                }]
            })
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
                    self.data.insert(id.number());
                }
                Event::AccountClosed(id) => {
                    self.data.remove(&id.number());
                }
                _ => {}
            }
        }
    }
}
