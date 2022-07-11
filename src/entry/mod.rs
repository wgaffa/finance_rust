use std::collections::BTreeMap;
use std::mem;

use chrono::prelude::*;

use crate::{
    account::{self, Category},
    balance::{Balance, TransactionMarker},
    error::JournalValidationError,
};

#[derive(Debug, Clone, PartialEq, Eq)]
struct EntryDetails {
    date: Date<Utc>,
    description: Option<String>,
}

/// An account with a name and identifier
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Account {
    number: account::Number,
    name: account::Name,
    category: Category,
}

impl Account {
    pub fn new<T: Into<account::Number>>(
        number: T,
        name: account::Name,
        element: Category,
    ) -> Self {
        Self {
            number: number.into(),
            name,
            category: element,
        }
    }

    pub fn number(&self) -> &account::Number {
        &self.number
    }

    pub fn name(&self) -> &account::Name {
        &self.name
    }

    pub fn category(&self) -> &Category {
        &self.category
    }
}

#[derive(Debug, Default)]
pub struct Chart {
    chart: BTreeMap<u32, Account>,
}

impl Chart {
    pub fn new() -> Self {
        Self {
            chart: BTreeMap::new(),
        }
    }

    /// Insert an account into the chart
    ///
    /// Returns the old value if it already contained a value, otherwise it returns None.
    pub fn insert(&mut self, account: Account) -> Option<Account> {
        match self.chart.get_mut(&account.number.number()) {
            Some(x) => {
                let old = mem::replace(x, account);
                Some(old)
            }
            None => {
                self.chart.insert(account.number.number(), account);
                None
            }
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = &Account> {
        self.chart.values()
    }
}

/// This describes a "line" in a journal and notes one account being affected
/// with a debit or credit transaction.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct JournalEntry<'a> {
    account: &'a Account,
    pub(crate) transaction: Balance,
}

impl<'a> JournalEntry<'a> {
    pub fn new<T: Into<Balance>>(account: &'a Account, transaction: T) -> Self {
        Self {
            account,
            transaction: transaction.into(),
        }
    }
    /// Returns a reference to the [Account] that is affected by this transaction
    pub fn account(&self) -> &Account {
        self.account
    }

    /// Get the transaction balance for this entry.
    pub fn balance(&self) -> &Balance {
        &self.transaction
    }
}

/// Journal is an entry into the bookkeeping.
///
/// This describes which accounts is being debited and which account is being credited
/// as well as the date and a description of the journal.
///
/// From <https://www.beginner-bookkeeping.com/bookkeeping-terms.html>
/// > An entry that is made into the accounts utilizing double entry bookkeeping to make
/// > an adjustment to the accounts such as if a correction has to be made.
/// > The journal describes which account is being debited and which account is being
/// > credited, the date, the reason for the journal and a reference.
#[derive(Debug, Clone)]
pub struct Journal<'a> {
    details: EntryDetails,
    entries: Vec<JournalEntry<'a>>,
}

impl<'a> Journal<'a> {
    pub fn new(date: Date<Utc>) -> Self {
        Self {
            details: EntryDetails {
                date,
                description: None,
            },
            entries: Vec::new(),
        }
    }

    pub fn set_description<T: Into<String>>(&mut self, description: T) {
        self.details.description = Some(description.into());
    }

    pub fn description(&self) -> Option<&String> {
        self.details.description.as_ref()
    }

    pub fn date(&self) -> &Date<Utc> {
        &self.details.date
    }

    pub fn push<T>(&mut self, account: &'a Account, transaction: T)
    where
        T: Into<Balance>,
    {
        self.entries.push(JournalEntry::new(account, transaction));
    }

    pub fn as_slice(&self) -> &[JournalEntry] {
        self.entries.as_slice()
    }

    pub fn iter(&self) -> impl Iterator<Item = &JournalEntry> {
        self.entries.iter()
    }

    pub fn validate(self) -> Result<ValidatedJournal<'a>, JournalValidationError> {
        let balance = self
            .entries
            .iter()
            .fold((0, 0), |(d, c), x| match &x.transaction {
                Balance::Credit(x) => (d, c + x.amount()),
                Balance::Debit(x) => (d + x.amount(), c),
            });

        if balance.0 == balance.1 {
            Ok(ValidatedJournal {
                details: self.details,
                entries: self.entries,
            })
        } else {
            Err(JournalValidationError {
                debit: TransactionMarker::debit(balance.0).unwrap(),
                credit: TransactionMarker::credit(balance.1).unwrap(),
            })
        }
    }
}

impl<'a> IntoIterator for Journal<'a> {
    type IntoIter = std::vec::IntoIter<JournalEntry<'a>>;
    type Item = JournalEntry<'a>;

    fn into_iter(self) -> Self::IntoIter {
        self.entries.into_iter()
    }
}

impl<'a> IntoIterator for &'a Journal<'a> {
    type IntoIter = std::slice::Iter<'a, JournalEntry<'a>>;
    type Item = &'a JournalEntry<'a>;

    fn into_iter(self) -> Self::IntoIter {
        self.entries.iter()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ValidatedJournal<'b> {
    details: EntryDetails,
    entries: Vec<JournalEntry<'b>>,
}

impl ValidatedJournal<'_> {
    pub fn description(&self) -> Option<&String> {
        self.details.description.as_ref()
    }

    pub fn date(&self) -> &Date<Utc> {
        &self.details.date
    }

    pub fn as_slice(&self) -> &[JournalEntry] {
        self.entries.as_slice()
    }

    pub fn iter(&self) -> impl Iterator<Item = &JournalEntry> {
        self.entries.iter()
    }
}

impl<'a> IntoIterator for ValidatedJournal<'a> {
    type IntoIter = std::vec::IntoIter<JournalEntry<'a>>;
    type Item = JournalEntry<'a>;

    fn into_iter(self) -> Self::IntoIter {
        self.entries.into_iter()
    }
}

impl<'a> IntoIterator for &'a ValidatedJournal<'a> {
    type IntoIter = std::slice::Iter<'a, JournalEntry<'a>>;
    type Item = &'a JournalEntry<'a>;

    fn into_iter(self) -> Self::IntoIter {
        self.entries.iter()
    }
}

impl PartialEq<Journal<'_>> for ValidatedJournal<'_> {
    fn eq(&self, other: &Journal<'_>) -> bool {
        self.details == other.details && self.entries == other.entries
    }
}

#[derive(Debug, Default)]
pub struct DayBook<'a> {
    journals: Vec<Journal<'a>>,
}

impl<'a> DayBook<'a> {
    pub fn new() -> Self {
        Self {
            journals: Vec::new(),
        }
    }

    pub fn push(&mut self, journal: Journal<'a>) {
        self.journals.push(journal);
    }
}

impl<'a> IntoIterator for DayBook<'a> {
    type IntoIter = std::vec::IntoIter<Journal<'a>>;
    type Item = Journal<'a>;

    fn into_iter(self) -> Self::IntoIter {
        self.journals.into_iter()
    }
}

impl<'a> IntoIterator for &'a DayBook<'_> {
    type IntoIter = std::slice::Iter<'a, Journal<'a>>;
    type Item = &'a Journal<'a>;

    fn into_iter(self) -> Self::IntoIter {
        self.journals.iter()
    }
}

#[cfg(test)]
mod test {
    use std::any::Any;

    use super::*;
    use test_case::test_case;

    use crate::{
        account,
        balance::{Credit, Debit, TransactionMarker},
    };

    pub fn is_debit(x: &dyn Any) -> bool {
        x.is::<TransactionMarker<Debit>>()
    }

    pub fn to_debit(x: &dyn Any) -> &TransactionMarker<Debit> {
        if let Some(tx) = x.downcast_ref() {
            tx
        } else {
            unimplemented!()
        }
    }

    pub fn to_credit(x: &dyn Any) -> &TransactionMarker<Credit> {
        if let Some(tx) = x.downcast_ref() {
            tx
        } else {
            unimplemented!()
        }
    }

    #[test_case(Transaction::debit(50).unwrap(), Transaction::debit(50).unwrap())]
    fn journal_entry_debit<T: 'static>(tx: TransactionMarker<T>, expected: TransactionMarker<Debit>)
    where
        TransactionMarker<T>: TransactionMarker,
    {
        let account = Account {
            name: account::Name::new(String::from("Test")).unwrap(),
            number: account::Number::new(54),
            category: Category::Asset,
        };

        let tx = if is_debit(&tx) {
            let debit = to_debit(&tx).to_owned();
            Balance::Debit(debit)
        } else {
            let credit = to_credit(&tx).to_owned();
            Balance::Credit(credit)
        };

        let actual = JournalEntry {
            account: &account,
            transaction: tx,
        };

        assert_eq!(actual.balance(), &Balance::Debit(expected));
    }

    #[test_case(Transaction::credit(50).unwrap(), Transaction::credit(50).unwrap())]
    fn journal_entry_credit<T: 'static, 'a>(tx: TransactionMarker<T>, expected: TransactionMarker<Credit>)
    where
        TransactionMarker<T>: TransactionMarker,
    {
        let account = Account {
            name: account::Name::new(String::from("Test")).unwrap(),
            number: account::Number::new(54),
            category: Category::Asset,
        };

        let tx = if is_debit(&tx) {
            let debit = to_debit(&tx).to_owned();
            Balance::Debit(debit)
        } else {
            let credit = to_credit(&tx).to_owned();
            Balance::Credit(credit)
        };

        let actual = JournalEntry {
            account: &account,
            transaction: tx,
        };

        assert_eq!(actual.balance(), &Balance::Credit(expected));
    }

    #[test]
    fn chart_insert_duplicate_gives_length_one() {
        let mut chart = Chart::new();

        chart.insert(Account::new(
            101,
            account::Name::new("Test").unwrap(),
            Category::Expenses,
        ));
        chart.insert(Account::new(
            101,
            account::Name::new("Duplicate number").unwrap(),
            Category::Asset,
        ));

        assert_eq!(chart.chart.len(), 1);
    }

    #[test]
    fn chart_insert_duplicate_returns_old() {
        let mut chart = Chart::new();

        chart.insert(Account::new(
            101,
            account::Name::new("Test").unwrap(),
            Category::Expenses,
        ));
        let actual = chart.insert(Account::new(
            101,
            account::Name::new("Duplicate number").unwrap(),
            Category::Asset,
        ));

        let expected = Account::new(101, account::Name::new("Test").unwrap(), Category::Expenses);

        assert_eq!(actual, Some(expected));
    }

    #[test]
    fn chart_insert_given_unique_account_returns_none() {
        let mut chart = Chart::new();

        let actual = chart.insert(Account::new(
            101,
            account::Name::new("Test").unwrap(),
            Category::Income,
        ));

        assert_eq!(actual, None);
    }

    #[test]
    fn chart_iter_empty() {
        let chart = Chart::new();

        let mut iter = chart.iter();

        assert_eq!(iter.next(), None);
    }

    #[test]
    fn chart_iter_single() {
        let mut chart = Chart::new();

        let account = Account::new(
            601,
            account::Name::new(String::from("Grocery")).unwrap(),
            Category::Expenses,
        );

        chart.insert(account.clone());

        let expected = vec![&account];

        let actual = chart.iter().collect::<Vec<_>>();

        assert_eq!(actual, expected);
    }

    #[test]
    fn chart_iter_multiple() {
        let mut chart = Chart::new();

        let mut accounts = vec![
            Account::new(
                201,
                account::Name::new("Credit Loan").unwrap(),
                Category::Liability,
            ),
            Account::new(401, account::Name::new("Salary").unwrap(), Category::Income),
            Account::new(
                502,
                account::Name::new("Phone").unwrap(),
                Category::Expenses,
            ),
            Account::new(
                501,
                account::Name::new("Internet").unwrap(),
                Category::Expenses,
            ),
            Account::new(
                202,
                account::Name::new("Bank Loan").unwrap(),
                Category::Liability,
            ),
            Account::new(
                101,
                account::Name::new("Bank Account").unwrap(),
                Category::Asset,
            ),
        ];

        for account in &accounts {
            chart.insert(account.clone());
        }

        accounts.sort();
        let mut expected = Vec::new();
        for account in &accounts {
            expected.push(account);
        }

        let actual = chart.iter().collect::<Vec<_>>();

        assert_eq!(actual, expected);
    }
}
