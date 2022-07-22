use personal_finance::account::Category;

pub mod store;

pub enum Event {
    AccountOpened(AccountOpened),
    AccountClosed(AccountClosed)
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct AccountOpened {
    id: u32,
    name: String,
    category: Category,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct AccountClosed(u32);
