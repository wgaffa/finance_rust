use chrono::prelude::*;
use personal_finance::{balance::Balance, account::{Category, Name, Number}};
use super::{JournalId, AccountId};

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
