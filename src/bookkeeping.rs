use std::any::Any;
use std::iter::Sum;
use std::marker::PhantomData;

#[derive(Debug, PartialEq)]
pub struct Debit;

#[derive(Debug, PartialEq)]
pub struct Credit;

pub trait TransactionMarker: Any {
    fn as_any(&self) -> &dyn Any;
}

impl<T: 'static> TransactionMarker for Transaction<T> {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

/// Data for a single transaction holding the entry type and amount
#[derive(Debug, PartialEq)]
pub struct Transaction<T> {
    amount: u32,
    phantom: PhantomData<T>,
}

impl<T> Transaction<T> {
    pub fn amount(&self) -> u32 {
        self.amount
    }

    pub fn map<F>(self, f: F) -> Self
    where
        F: Fn(u32) -> u32
    {
        Self {
            amount: f(self.amount),
            phantom: PhantomData,
        }
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

impl<'a, T> Sum<&'a Self> for Transaction<T> {
    fn sum<I>(iter: I) -> Self
    where
        I: Iterator<Item = &'a Self>
    {
        iter.fold(Self { amount: 0, phantom: PhantomData }, |acc, el| Self {
            amount: acc.amount + el.amount,
            phantom: PhantomData,
        })
    }
}

impl<'a, T> Sum for Transaction<T> {
    fn sum<I>(iter: I) -> Self
    where
        I: Iterator<Item = Self>,
    {
        iter.fold(Self { amount: 0, phantom: PhantomData }, |acc, el| Self {
            amount: acc.amount + el.amount,
            phantom: PhantomData,
        })
    }
}

/// Split a vector of Any trait objects into its Debit and Credit
///
/// This returns a tuple where the first one is the debits and second is credits
///
/// # Panics
/// If the vector contains other types than `Transaction<Debit>` or `Transaction<Credit>`
pub fn split(collection: Vec<Box<dyn Any>>) -> (Vec<Transaction<Debit>>, Vec<Transaction<Credit>>) {
    let (debits, credits): (Vec<Box<dyn Any>>, Vec<Box<dyn Any>>) =
        collection
        .into_iter()
        .partition(|x| x.is::<Transaction<Debit>>());

    // Since we split the transactions on the Debit types we can just unwrap the debits downcast.
    // But credits may contain malicious types and we therefore inspect the elements and panics if
    // it's not a credit type. Otherwise we just move on.
    let debits = debits
        .into_iter()
        .map(|x| *x.downcast::<Transaction<Debit>>().unwrap())
        .collect::<Vec<Transaction<Debit>>>();
    let credits = credits
        .into_iter()
        .map(|x| {
            match x.downcast::<Transaction<Credit>>() {
                Ok(c) => *c,
                Err(_e) => panic!("Trying to split trait objects of incompatible types"),
            }
        })
        .collect::<Vec<Transaction<Credit>>>();

    (debits, credits)
}

#[cfg(test)]
#[path ="bookkeeping_tests.rs"]
mod bookkeeping_tests;
