use std::{collections::HashMap, ops::Deref, sync::Arc};

use personal_finance::account::{Category, Name, Number};

use crate::{AccountError, Event, events::EventPointer};

use super::ledger::LedgerId;

type IsOpen = bool;
#[derive(Default)]
pub struct Chart {
    data: HashMap<Number, IsOpen>,
    history: Vec<EventPointer>,
}

impl Chart {
    pub fn new(history: &[EventPointer]) -> Self {
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
    ) -> Result<&[EventPointer], AccountError> {
        let account_doesnt_exist = !self.data.contains_key(&number);
        account_doesnt_exist
            .then_some(())
            .ok_or_else(|| {
                if *self.data.get(&number).unwrap() {
                    AccountError::Opened(number.number())
                } else {
                    AccountError::NotExist
                }
            })
            .map(|()| {
                vec![Arc::new(Event::AccountOpened {
                    ledger: LedgerId::new("Bogus").unwrap(),
                    id: number,
                    name,
                    category,
                })]
            })
            .map(|issued_events| self.apply_new_events(issued_events))
    }

    pub fn close(&mut self, number: Number) -> Result<&[EventPointer], AccountError> {
        let account_exists_and_opened = self.data.get(&number).copied().unwrap_or_default();
        account_exists_and_opened
            .then_some(())
            .map(|()| {
                vec![Arc::new(Event::AccountClosed {
                    ledger: LedgerId::new("Bogus").unwrap(),
                    account: number,
                })]
            })
            .map(|issued_events| self.apply_new_events(issued_events))
            .ok_or(AccountError::Closed)
    }

    fn apply(&mut self, events: &[EventPointer]) {
        for event in events {
            match event.deref() {
                Event::AccountOpened { id, .. } => {
                    self.data.insert(*id, true);
                }
                Event::AccountClosed { account: id, .. } => {
                    self.data.entry(*id).and_modify(|x| *x = false);
                }
                _ => {}
            }
        }
    }

    fn apply_new_events(&mut self, events: Vec<EventPointer>) -> &[EventPointer] {
        let len = events.len();
        self.apply(&events);
        self.history.extend(events);

        let index = self.history.len().checked_sub(len).unwrap_or_default();
        &self.history[index..]
    }
}
