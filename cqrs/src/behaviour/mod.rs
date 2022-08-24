use crate::events::Event;
use personal_finance::account::Category;

#[derive(Debug)]
pub enum AccountError {
    AccountAlreadyOpened(u32),
}

impl std::fmt::Display for AccountError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::AccountAlreadyOpened(id) => write!(f, "Account '{id}' has already been opened"),
        }
    }
}

impl std::error::Error for AccountError {}

pub fn open_account(
    id: u32,
    name: String,
    category: Category,
    _events: &[Event],
) -> Vec<Event> {
    vec![Event::AccountOpened { id, name, category: category }]
}
