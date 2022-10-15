use std::fmt;

use thiserror::Error;

#[derive(Debug, PartialEq, Eq, Error)]
pub enum AccountError {
    #[error("Account '{0}' has already been opened.")]
    Opened(u32),
    #[error("Account has already been closed.")]
    Closed,
    #[error("Account doesn't exist.")]
    NotExist,
}

#[non_exhaustive]
#[derive(Debug, PartialEq, Eq)]
pub enum JournalError {
    JournalLimitReached,
    ImbalancedTranasactions,
    EmptyTransaction,
    NoJournalEvent,
    InvalidTransaction,
}

impl fmt::Display for JournalError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::JournalLimitReached => f.write_str("The maximum journal id exceeded"),
            Self::ImbalancedTranasactions => {
                f.write_str("The balance of the transactions does not equal zero")
            }
            Self::EmptyTransaction => f.write_str("A journal must have atleast one transaction"),
            Self::NoJournalEvent => f.write_str("No journal event in the stream"),
            Self::InvalidTransaction => f.write_str("The transaction was not valid"),
        }
    }
}
