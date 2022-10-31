use std::sync::Arc;

use super::JournalId;
use crate::write::ledger::LedgerId;
use chrono::prelude::*;
use personal_finance::{
    account::{Category, Name, Number},
    balance::Balance,
};

pub mod projections;
pub mod store;

pub type EventPointer = Arc<Event>;

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
        description: String,
        date: Date<Utc>,
        transactions: Vec<(Number, Balance)>,
    },
    #[deprecated(note = "This will be removed and replaced with Transaction in a Ledger context")]
    Journal {
        ledger: LedgerId,
        id: JournalId,
        description: String,
        date: Date<Utc>,
    },
}
