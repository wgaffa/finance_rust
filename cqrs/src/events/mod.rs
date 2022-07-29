use personal_finance::{account::Category, balance::Balance};

pub mod store;

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
