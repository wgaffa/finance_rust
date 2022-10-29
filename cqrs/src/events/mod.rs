use super::JournalId;
use crate::write::ledger::LedgerId;
use chrono::prelude::*;
use personal_finance::{
    account::{Category, Name, Number},
    balance::Balance,
};

pub mod projections;
pub mod store;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Event {
    LedgerCreated {
        id: LedgerId,
    },
    AccountOpened {
        ledger: LedgerId,
        id: Number,
        name: Name,
        category: Category,
    },
    AccountClosed {
        ledger: LedgerId,
        account: Number,
    },
    Transaction {
        ledger: LedgerId,
        account: Number,
        amount: Balance,
        journal: JournalId,
    },
    #[deprecated(note = "This will be removed and replaced with Transaction in a Ledger context")]
    Journal {
        ledger: LedgerId,
        id: JournalId,
        description: String,
        date: Date<Utc>,
    },
}
