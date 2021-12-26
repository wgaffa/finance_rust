use chrono::prelude::*;

use crate::bookkeeping::TransactionMarker;
use crate::entry::{Account, Journal};

struct LedgerEntry<'a> {
    date: &'a Date<Utc>,
    transaction: &'a dyn TransactionMarker,
}

pub struct Ledger<'a> {
    account: &'a Account,
    entries: Vec<LedgerEntry<'a>>,
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
    pub fn push(&mut self, journal: &'a Journal) {
        for entry in journal.iter() {
            if entry.account() == self.account {
                let ledger_entry = LedgerEntry {
                    date: journal.date(),
                    transaction: entry.transaction.as_ref(),
                };

                self.entries.push(ledger_entry);
            }
        }
    }
}