use std::fmt;

#[derive(Debug, PartialEq, Eq)]
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

#[derive(Debug, PartialEq, Eq)]
pub enum JournalError {
    JournalLimitReached,
    ImbalancedTranasactions,
    EmptyTransaction,
}

impl fmt::Display for JournalError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::JournalLimitReached => f.write_str("The maximum journal id exceeded"),
            Self::ImbalancedTranasactions => f.write_str("The balance of the transactions does not equal zero"),
            Self::EmptyTransaction => f.write_str("A journal must have atleast one transaction"),
        }
    }
}
