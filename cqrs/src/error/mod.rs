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
#[derive(Debug, PartialEq, Eq, Error)]
pub enum TransactionError {
    #[error("The balance of the transactions does not equal zero")]
    ImbalancedTranasactions,
    #[error("A journal must have atleast one transaction")]
    EmptyTransaction,
    #[error("Could not add a transaction to specified account")]
    AccountDoesntExist,
    #[error("That ledger doesn't exist")]
    LedgerDoesnExist,
}
