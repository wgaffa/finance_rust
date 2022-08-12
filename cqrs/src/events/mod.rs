use personal_finance::account::Category;

pub mod store;
pub mod projections;

pub type JournalId = u32;
pub type AccountId = u32;

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
        id: u32,
        description: JournalId,
    }
}
