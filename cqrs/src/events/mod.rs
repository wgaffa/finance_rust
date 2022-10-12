use super::JournalId;
use chrono::prelude::*;
use personal_finance::{
    account::{Category, Name, Number},
    balance::Balance,
};

pub mod projections;
pub mod store;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Event {
    AccountOpened {
        id: Number,
        name: Name,
        category: Category,
    },
    AccountClosed(Number),
    Transaction {
        account: Number,
        amount: Balance,
        journal: JournalId,
    },
    Journal {
        id: JournalId,
        description: String,
        date: Date<Utc>,
    },
}
