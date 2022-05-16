use std::any::Any;
use std::iter::Sum;
use std::marker::PhantomData;

/// A balance is either a Debit or Credit transaction
///
/// # Examples
/// ```
/// use personal_finance::balance::{Transaction, Balance};
///
/// let debit = Balance::debit(50);
/// let credit = Balance::credit(20);
///
/// assert_eq!(debit, Balance::Debit(Transaction::debit(50)));
/// assert_eq!(credit, Balance::Credit(Transaction::credit(20)));
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Balance {
    Debit(Transaction<Debit>),
    Credit(Transaction<Credit>),
}

impl Balance {
    /// Create a new debit balance
    pub fn debit(amount: u32) -> Self {
        Self::Debit(Transaction::debit(amount))
    }

    /// Create a new credit balance
    pub fn credit(amount: u32) -> Self {
        Self::Credit(Transaction::credit(amount))
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

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Debit;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Credit;

pub(crate) trait TransactionMarker: Any + std::fmt::Debug {
    fn as_any(&self) -> &dyn Any;

    fn as_debit(&self) -> Option<&Transaction<Debit>> {
        None
    }

    fn as_credit(&self) -> Option<&Transaction<Credit>> {
        None
    }

    fn as_balance(&self) -> Option<Balance> {
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

    fn as_balance(&self) -> Option<Balance> {
        Some(Balance::Credit(self.to_owned()))
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

    fn as_balance(&self) -> Option<Balance> {
        Some(Balance::Debit(self.to_owned()))
    }
}

/// Data for a single transaction holding the entry type and amount
#[derive(Debug, Clone, PartialEq, Eq)]
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
    /// use personal_finance::balance::Transaction;
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
pub fn split(collection: Vec<Balance>) -> (Vec<Transaction<Debit>>, Vec<Transaction<Credit>>) {
    #[allow(clippy::type_complexity)]
    let (debits, credits): (
        Vec<Box<dyn TransactionMarker>>,
        Vec<Box<dyn TransactionMarker>>,
    ) = collection
        .into_iter()
        .map(|x| match x {
            Balance::Credit(credit) => Box::new(credit) as Box<dyn TransactionMarker>,
            Balance::Debit(debit) => Box::new(debit),
        })
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
        .map(|x| {
            x.as_credit()
                .expect("Trying to split trait objects of incompatible types")
                .to_owned()
        })
        .collect::<Vec<Transaction<Credit>>>();

    (debits, credits)
}

#[cfg(test)]
mod tests;
