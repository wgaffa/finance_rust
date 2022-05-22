use thiserror::Error;

use crate::balance::{Credit, Debit, Transaction};

#[derive(Debug, Error)]
#[error("mismatched debit {debit:?} and credit {credit:?} balances")]
pub struct JournalValidationError {
    pub(crate) debit: Transaction<Debit>,
    pub(crate) credit: Transaction<Credit>,
}

impl JournalValidationError {
    pub fn debit(&self) -> &Transaction<Debit> {
        &self.debit
    }

    pub fn credit(&self) -> &Transaction<Credit> {
        &self.credit
    }
}
