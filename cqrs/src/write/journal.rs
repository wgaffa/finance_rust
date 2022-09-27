use std::{collections::HashSet, ops::Neg};

use crate::{AccountId, Balance, Event, JournalError, JournalId};

#[derive(Default)]
pub struct Journal {
    current_id: JournalId,
    accounts: HashSet<AccountId>,
    history: Vec<Event>,
}

fn transcribe_amount(amount: Balance) -> i64 {
    match amount {
        Balance::Debit(x) => i64::from(x),
        Balance::Credit(x) => i64::from(x).neg(),
    }
}

fn make_journal(
    id: JournalId,
    description: String,
    transactions: &[(AccountId, Balance)],
) -> Vec<Event> {
    let mut v = vec![Event::Journal {
        id,
        description: description.into(),
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
        transactions: &[(AccountId, Balance)],
    ) -> Result<&[Event], JournalError> {
        transactions
            .len()
            .gt(&0)
            .then_some(())
            .ok_or(JournalError::EmptyTransaction)
            .and_then(|()| {
                transactions
                    .iter()
                    .fold(0, |sum, (_, amount)| sum + transcribe_amount(*amount))
                    .eq(&0)
                    .then_some(())
                    .ok_or(JournalError::ImbalancedTranasactions)
                    .and_then(|()| next_id(self.current_id))
                    .map(|id| make_journal(id, description.into(), &transactions))
                    .map(|events| {
                        self.apply(&events);
                        let len = self.history.len();
                        self.history.extend(events);
                        len
                    })
                    .map(|len| &self.history[len..])
            })
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

