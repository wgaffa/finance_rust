use personal_finance::{account::Category, balance::Balance};

pub mod store;
pub mod projections;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Event {
    AccountOpened {
        id: u32,
        name: String,
        category: Category,
    },
    AccountClosed(u32),
    Transaction {
        account: u32,
        amount: Balance,
    },
}
