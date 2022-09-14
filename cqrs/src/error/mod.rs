use std::fmt;

#[derive(Debug)]
pub enum AccountError {
    AccountAlreadyOpened(u32),
}

impl fmt::Display for AccountError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::AccountAlreadyOpened(id) => write!(f, "Account '{id}' has already been opened"),
        }
    }
}

impl std::error::Error for AccountError {}

#[derive(Debug)]
pub enum JournalError {
    JournalLimitReached,
}

impl fmt::Display for JournalError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::JournalLimitReached => f.write_str("The maximum journal id exceeded"),
        }
    }
}
