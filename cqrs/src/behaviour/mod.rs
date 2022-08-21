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

type BehaviourError<E> = error_stack::Result<Vec<Event>, E>;

pub fn open_account(
    id: u32,
    name: String,
    category: Category,
    events: &[Event],
) -> BehaviourError<AccountError> {
    if events
        .iter()
        .any(|e| matches!(e, Event::AccountOpened { id: e_id, .. } if *e_id == id ))
    {
        error_stack::bail!(AccountError::AccountAlreadyOpened(id))
    }

    Ok(vec![Event::AccountOpened { id, name, category }])
}
