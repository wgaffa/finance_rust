use std::marker::PhantomData;

/// Entry type for bookkeeping
#[derive(Debug, PartialEq)]
pub struct Debit;

#[derive(Debug, PartialEq)]
pub struct Credit;

/// Data for a single transaction holding the entry type and amount
pub struct Transaction<T> {
    amount: u32,
    phantom: PhantomData<T>,
}

impl<T> Transaction<T> {
    pub fn amount(&self) -> u32 {
        self.amount
    }
}

impl Transaction::<Debit> {
    /// Create a new debit transaction
    ///
    /// ```
    /// use personal_finance::bookkeeping::Transaction;
    /// let transaction = Transaction::debit(40);
    /// assert_eq!(transaction.amount(), 40);
    /// ```
    pub fn debit(amount: u32) -> Self {
        Self {
            amount,
            phantom: PhantomData,
        }
    }
}

impl Transaction::<Credit> {
    /// Create a new credit transaction
    ///
    /// ```
    /// use personal_finance::bookkeeping::Transaction;
    /// let transaction = Transaction::credit(70);
    /// assert_eq!(transaction.amount(), 70);
    /// ```
    pub fn credit(amount: u32) -> Self {
        Self {
            amount,
            phantom: PhantomData,
        }
    }
}

#[cfg(test)]
#[path ="bookkeeping_tests.rs"]
mod bookkeeping_tests;
