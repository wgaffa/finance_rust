/// Entry type for bookkeeping
#[derive(Debug, PartialEq)]
pub enum Entry {
    Debit,
    Credit,
}

type AmountType = Amount;

#[derive(Debug)]
pub struct Amount(i32);

impl Amount {
    pub fn new(amount: i32) -> Self {
        Self (amount.abs())
    }
}

/// Data for a single transaction holding the entry type and amount
pub struct Transaction {
    entry: Entry,
    amount: AmountType,
}

impl Transaction {
    /// Create a new debit transaction
    ///
    /// ```
    /// use personal_finance::bookkeeping::{Transaction, Amount};
    /// let transaction = Transaction::debit(Amount::new(40));
    /// assert_eq!(transaction.amount(), 40);
    /// ```
    pub fn debit(amount: AmountType) -> Self {
        Self {
            entry: Entry::Debit,
            amount,
        }
    }

    /// Create a new credit transaction
    ///
    /// ```
    /// use personal_finance::bookkeeping::{Transaction, Amount};
    /// let transaction = Transaction::credit(Amount::new(70));
    /// assert_eq!(transaction.amount(), 70);
    /// ```
    pub fn credit(amount: AmountType) -> Self {
        Self {
            entry: Entry::Credit,
            amount,
        }
    }

    pub fn amount(&self) -> i32 {
        self.amount.0
    }

    pub fn entry(&self) -> &Entry {
        &self.entry
    }

    pub fn to_numeral(&self) -> i32 {
        self.amount.0 * match self.entry {
            Entry::Debit => 1,
            Entry::Credit => -1,
        }
    }
}

fn out_of_debt(e: Entry) {
}

#[cfg(test)]
#[path ="bookkeeping_tests.rs"]
mod bookkeeping_tests;
