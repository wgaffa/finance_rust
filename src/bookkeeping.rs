use std::any::{Any, TypeId};
use std::iter::Sum;
use std::marker::PhantomData;

pub fn is_debit<T: ?Sized + Any>(_s: &T) -> bool {
    TypeId::of::<Transaction<Debit>>() == TypeId::of::<T>()
}

pub fn is_credit<T: ?Sized + Any>(_s: &T) -> bool {
    TypeId::of::<Transaction<Credit>>() == TypeId::of::<T>()
}

pub enum Balance {
    Debit(Transaction<Debit>),
    Credit(Transaction<Credit>),
}

pub fn to_balance<T: TransactionMarker>(value: T) -> Balance {
    if is_debit(&value) {
        Balance::Debit(value.as_debit().unwrap().to_owned())
    } else if is_credit(&value) {
        Balance::Credit(value.as_credit().unwrap().to_owned())
    } else {
        panic!("Could not convert to a balance")
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Debit;

#[derive(Debug, Clone, PartialEq)]
pub struct Credit;

pub trait TransactionMarker: Any {
    fn as_any(&self) -> &dyn Any;

    fn as_debit(&self) -> Option<&Transaction<Debit>> {
        None
    }

    fn as_credit(&self) -> Option<&Transaction<Credit>> {
        None
    }
}

impl TransactionMarker for Transaction<Credit> {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_credit(&self) -> Option<&Transaction<Credit>>
    where
        Self: Sized,
    {
        Some(self)
    }
}

impl TransactionMarker for Transaction<Debit> {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_debit(&self) -> Option<&Transaction<Debit>>
    where
        Self: Sized,
    {
        Some(self)
    }
}

/// Data for a single transaction holding the entry type and amount
#[derive(Debug, Clone, PartialEq)]
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
        F: Fn(u32) -> u32,
    {
        Self {
            amount: f(self.amount),
            phantom: PhantomData,
        }
    }
}

impl Transaction<Debit> {
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

impl Transaction<Credit> {
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
        I: Iterator<Item = &'a Self>,
    {
        iter.fold(
            Self {
                amount: 0,
                phantom: PhantomData,
            },
            |acc, el| Self {
                amount: acc.amount + el.amount,
                phantom: PhantomData,
            },
        )
    }
}

impl<'a, T> Sum for Transaction<T> {
    fn sum<I>(iter: I) -> Self
    where
        I: Iterator<Item = Self>,
    {
        iter.fold(
            Self {
                amount: 0,
                phantom: PhantomData,
            },
            |acc, el| Self {
                amount: acc.amount + el.amount,
                phantom: PhantomData,
            },
        )
    }
}

/// Split a vector of Any trait objects into its Debit and Credit
///
/// This returns a tuple where the first one is the debits and second is credits
///
/// # Panics
/// If the vector contains other types than `Transaction<Debit>` or `Transaction<Credit>`
pub fn split(collection: Vec<Box<dyn TransactionMarker>>) -> (Vec<Transaction<Debit>>, Vec<Transaction<Credit>>) {
    #[allow(clippy::type_complexity)]
    let (debits, credits): (Vec<Box<dyn TransactionMarker>>, Vec<Box<dyn TransactionMarker>>) = collection
.into_iter()
        .partition(|x| x.as_any().is::<Transaction<Debit>>());

    // Since we split the transactions on the Debit types we can just unwrap the debits downcast.
    // But credits may contain malicious types and we therefore inspect the elements and panics if
    // it's not a credit type. Otherwise we just move on.
    let debits = debits
        .into_iter()
        // .map(|x| x.as_any().downcast_ref::<Transaction<Debit>>().unwrap().to_owned())
        .map(|x| x.as_debit().unwrap().to_owned())
        .collect::<Vec<Transaction<Debit>>>();
    let credits = credits
        .into_iter()
        .map(
            |x|
            x
                .as_credit()
                .expect("Trying to split trait objects of incompatible types")
                .to_owned()
        )
        .collect::<Vec<Transaction<Credit>>>();

    (debits, credits)
}

#[cfg(test)]
#[path = "bookkeeping_tests.rs"]
mod bookkeeping_tests;
