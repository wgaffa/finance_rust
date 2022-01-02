use chrono::prelude::*;

use crate::balance::{TransactionMarker, Balance};
use crate::entry::{Account, Journal};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LedgerEntry {
    date: Date<Utc>,
    transaction: Balance,
}

#[derive(Debug)]
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
    pub fn push(&mut self, journal: &'a Journal) -> usize {
        let mut count = 0;
        for entry in journal {
            if entry.account() == self.account {
                let ledger_entry = LedgerEntry {
                    date: journal.date().to_owned(),
                    transaction: entry.transaction.as_balance().unwrap(),
                };

                self.entries.push(ledger_entry);
                count += 1;
            }
        }

        count
    }

    pub fn iter(&self) -> impl Iterator<Item = &LedgerEntry> {
        self.entries.iter()
    }
}

pub struct Iter<'a> {
    slice: &'a [LedgerEntry],
}

impl<'a> Iterator for Iter<'a> {
    type Item = (&'a Date<Utc>, &'a Balance);

    fn next(&mut self) -> Option<Self::Item> {
        None
    }
}

#[cfg(test)]
mod tests;
