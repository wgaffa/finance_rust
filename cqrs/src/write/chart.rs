use std::collections::HashSet;

use personal_finance::account::{Category, Name, Number};

use crate::{AccountError, Event};

#[derive(Default)]
pub struct Chart {
    data: HashSet<Number>,
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
        let account_doesnt_exist = !self.data.contains(&number);
        account_doesnt_exist
            .then_some(())
            .map(|()| {
                vec![Event::AccountOpened {
                    id: number,
                    name,
                    category,
                }]
            })
            .map(|issued_events| self.apply_new_events(issued_events))
            .ok_or_else(|| AccountError::AccountAlreadyOpened(number.number()))
    }

    pub fn close(&mut self, number: Number) -> Result<&[Event], AccountError> {
        let account_exists = self.data.contains(&number);
        account_exists
            .then_some(())
            .map(|()| vec![Event::AccountClosed(number)])
            .map(|issued_events| self.apply_new_events(issued_events))
            .ok_or(AccountError::AccountAlreadyClosed)
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

    fn apply_new_events(&mut self, events: Vec<Event>) -> &[Event] {
        let len = events.len();
        self.apply(&events);
        self.history.extend(events);

        let index = self.history.len().checked_sub(len).unwrap_or_default();
        &self.history[index..]
    }
}
