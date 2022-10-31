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
    #[error("That ledger doesn't exist")]
    LedgerDoesnExist,
}

#[derive(Debug, PartialEq, Eq, Error)]
pub enum LedgerError {
    #[error("The ledger already exists")]
    AlreadyExists,
}

#[non_exhaustive]
#[derive(Debug, PartialEq, Eq)]
pub enum TransactionError {
    ImbalancedTranasactions,
    EmptyTransaction,
    AccountDoesntExist,
}

impl fmt::Display for TransactionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ImbalancedTranasactions => {
                f.write_str("The balance of the transactions does not equal zero")
            }
            Self::EmptyTransaction => f.write_str("A journal must have atleast one transaction"),
            Self::AccountDoesntExist => f.write_str("Could not add a transaction to specified account"),
        }
    }
}
