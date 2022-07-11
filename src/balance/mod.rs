use std::{any::Any, convert::TryInto, iter::Sum, marker::PhantomData, num::NonZeroU32};

/// A balance is either a Debit or Credit transaction
///
/// # Examples
/// ```
/// use personal_finance::balance::{Transaction, Balance};
///
/// let debit = Balance::debit(50).unwrap();
/// let credit = Balance::credit(20).unwrap();
///
/// assert_eq!(debit.amount(), 50);
/// assert_eq!(credit.amount(), 20);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Balance {
    Debit(Transaction<Debit>),
    Credit(Transaction<Credit>),
}

impl Balance {
    /// Create a new debit balance
    pub fn debit<T: TryInto<NonZeroU32>>(amount: T) -> Option<Self> {
        amount
            .try_into()
            .map(|x| Self::Debit(Transaction::debit_unchecked(x.into())))
            .ok()
    }

    /// Create a new credit balance
    pub fn credit<T: TryInto<NonZeroU32>>(amount: T) -> Option<Self> {
        amount
            .try_into()
            .map(|x| Self::Credit(Transaction::credit_unchecked(x.into())))
            .ok()
    }

    /// Get the amount of either the debit or credit
    pub fn amount(&self) -> u32 {
        match self {
            Balance::Debit(x) => x.amount(),
            Balance::Credit(x) => x.amount(),
        }
    }
}

impl From<Transaction<Debit>> for Balance {
    fn from(value: Transaction<Debit>) -> Self {
        Self::Debit(value)
    }
}

impl From<Transaction<Credit>> for Balance {
    fn from(value: Transaction<Credit>) -> Self {
        Self::Credit(value)
    }
}

impl From<Box<Transaction<Debit>>> for Balance {
    fn from(value: Box<Transaction<Debit>>) -> Self {
        Self::Debit(Box::into_inner(value))
    }
}

impl From<Box<Transaction<Credit>>> for Balance {
    fn from(value: Box<Transaction<Credit>>) -> Self {
        Self::Credit(Box::into_inner(value))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Debit;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Credit;

pub(crate) trait TransactionMarker: std::fmt::Debug {
    fn as_any(&self) -> &dyn Any;

    fn as_balance(&self) -> Balance;
}

impl TransactionMarker for Transaction<Credit> {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_balance(&self) -> Balance {
        Balance::Credit(self.to_owned())
    }
}

impl TransactionMarker for Transaction<Debit> {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_balance(&self) -> Balance {
        Balance::Debit(self.to_owned())
    }
}

/// Data for a single transaction holding the entry type and amount
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
    /// # Examples
    /// ```
    /// use personal_finance::balance::Transaction;
    /// let transaction = Transaction::debit(40).unwrap();
    /// assert_eq!(transaction.amount(), 40);
    /// ```
    pub fn debit<T: TryInto<NonZeroU32>>(amount: T) -> Option<Self> {
        amount
            .try_into()
            .map(|amount| Self {
                amount: amount.into(),
                phantom: PhantomData,
            })
            .ok()
    }

    pub(crate) fn debit_unchecked(amount: u32) -> Self {
        assert!(amount != 0);

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
    /// use personal_finance::balance::Transaction;
    /// let transaction = Transaction::credit(70).unwrap();
    /// assert_eq!(transaction.amount(), 70);
    /// ```
    pub fn credit<T: TryInto<NonZeroU32>>(amount: T) -> Option<Self> {
        amount
            .try_into()
            .map(|amount| Self {
                amount: amount.into(),
                phantom: PhantomData,
            })
            .ok()
    }

    pub(crate) fn credit_unchecked(amount: u32) -> Self {
        assert!(amount != 0);

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
            |acc, el| acc + el,
        )
    }
}

impl<T> Sum for Transaction<T> {
    fn sum<I>(iter: I) -> Self
    where
        I: Iterator<Item = Self>,
    {
        iter.fold(
            Self {
                amount: 0,
                phantom: PhantomData,
            },
            |acc, el| acc + el,
        )
    }
}

impl<T> std::ops::Add for Transaction<T> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            amount: self.amount + rhs.amount,
            phantom: PhantomData,
        }
    }
}

impl<T> std::ops::Add<&Transaction<T>> for Transaction<T> {
    type Output = Self;

    fn add(self, rhs: &Self) -> Self::Output {
        Self {
            amount: self.amount + rhs.amount,
            phantom: PhantomData,
        }
    }
}

impl<T> std::ops::AddAssign for Transaction<T> {
    fn add_assign(&mut self, rhs: Self) {
        self.amount += rhs.amount;
    }
}

/// Split a vector of Any trait objects into its Debit and Credit
///
/// This returns a tuple where the first one is the debits and second is credits
///
/// # Panics
/// If the vector contains other types than `Transaction<Debit>` or `Transaction<Credit>`
pub fn split<I>(collection: I) -> (Vec<Transaction<Debit>>, Vec<Transaction<Credit>>)
where
    I: IntoIterator<Item = Balance>,
{
    collection
        .into_iter()
        .fold((Vec::new(), Vec::new()), |mut tup, x| match x {
            Balance::Credit(credit) => { tup.1.push(credit); (tup.0, tup.1) },
            Balance::Debit(debit) => { tup.0.push(debit); (tup.0, tup.1) },
        })
}

#[cfg(test)]
mod tests;
