use personal_finance::account::Category;
use super::{JournalId, AccountId};

pub mod projections;
pub mod store;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Balance {
    Debit(u32),
    Credit(u32),
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Event {
    AccountOpened {
        id: AccountId,
        name: String,
        category: Category,
    },
    AccountClosed(u32),
    Transaction {
        account: AccountId,
        amount: Balance,
        journal: JournalId,
    },
    Journal {
        id: JournalId,
        description: String,
    },
}
