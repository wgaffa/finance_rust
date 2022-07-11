use chrono::prelude::*;

use crate::balance::Balance;
use crate::entry::{Account, ValidatedJournal};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LedgerEntry {
    date: Date<Utc>,
    transaction: Balance,
}

#[derive(Debug, Clone)]
pub struct Ledger<'a> {
    account: &'a Account,
    entries: Vec<LedgerEntry>,
}

impl<'a> Ledger<'a> {
    pub fn new(account: &'a Account) -> Self {
        Self {
            account,
            entries: Vec::new(),
        }
    }

    /// Push an entry in the ledger only if the entry is for
    /// the same account
    pub fn push(&mut self, journal: ValidatedJournal) -> usize {
        let mut count = 0;
        let date = journal.date().to_owned();
        for entry in journal {
            if entry.account() == self.account {
                let ledger_entry = LedgerEntry {
                    date,
                    transaction: entry.transaction,
                };

                self.entries.push(ledger_entry);
                count += 1;
            }
        }

        count
    }

    pub fn iter(&self) -> Iter<'_> {
        Iter::new(&self.entries)
    }
}

pub struct Iter<'a> {
    slice: &'a [LedgerEntry],
}

impl<'a> Iter<'a> {
    fn new(slice: &'a [LedgerEntry]) -> Self {
        Self { slice }
    }
}

impl<'a> Iterator for Iter<'a> {
    type Item = (&'a Date<Utc>, &'a Balance);

    fn next(&mut self) -> Option<Self::Item> {
        let (item, rest) = self.slice.split_first()?;
        self.slice = rest;
        Some((&item.date, &item.transaction))
    }
}

#[cfg(test)]
mod tests;
