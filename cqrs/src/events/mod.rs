pub enum Event {
    AccountOpened(AccountOpened),
    AccountClosed(AccountClosed)
}

pub struct AccountOpened {
    id: u32,
    name: String,
    category: String,
}

pub struct AccountClosed(u32);
