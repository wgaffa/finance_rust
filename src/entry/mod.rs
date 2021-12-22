use std::collections::{BTreeMap, BTreeSet};
use chrono::prelude::*;
use enum_iterator::IntoEnumIterator;

use crate::bookkeeping::{Credit, Debit, Transaction, TransactionMarker};

#[derive(Debug)]
struct EntryDetails {
    date: Date<Utc>,
    description: Option<String>,
}

/// An account number to identify an account.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct AccountNumber(u32);

impl AccountNumber {
    /// Create a new [AccountNumber] with a positive integer
    pub fn new(value: u32) -> Self {
        Self(value)
    }
}

impl std::convert::From<u32> for AccountNumber {
    fn from(value: u32) -> Self {
        Self(value)
    }
}

impl std::convert::From<AccountNumber> for u32 {
    fn from(number: AccountNumber) -> Self {
        number.0
    }
}

impl std::fmt::Display for AccountNumber {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// An account name is a trimmed non-empty string.
///
/// # Examples
/// ```
/// use personal_finance::entry::AccountName;
///
/// let name = AccountName::new("  My Bank Account\n");
/// assert_eq!(name.unwrap(), "My Bank Account");
///
/// let name = AccountName::new("    ");
/// assert_eq!(name, None);
/// ```
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct AccountName(String);

impl AccountName {
    /// Create a new AccountName
    ///
    /// This trims and returns Some(AccountName) if it is not an empty string,
    /// otherwise it return None.
    pub fn new<T: AsRef<str>>(name: T) -> Option<Self> {
        let name = name.as_ref().trim().to_owned();
        if name.is_empty() {
            None
        } else {
            Some(AccountName(name))
        }
    }

    /// Move the inner string out of AccountName thus consuming it
    pub fn into_inner(self) -> String {
        self.0
    }
}

impl PartialEq<String> for AccountName {
    fn eq(&self, other: &String) -> bool {
        self.0 == *other
    }
}

impl PartialEq<&str> for AccountName {
    fn eq(&self, other: &&str) -> bool {
        self.0 == *other
    }
}

impl PartialOrd<String> for AccountName {
    fn partial_cmp(&self, other: &String) -> Option<std::cmp::Ordering> {
        self.0.partial_cmp(other)
    }
}

impl PartialOrd<&str> for AccountName {
    fn partial_cmp(&self, other: &&str) -> Option<std::cmp::Ordering> {
        self.0.as_str().partial_cmp(*other)
    }
}

/// These are the different types of an Account can be associated with.
#[derive(Debug, Clone, IntoEnumIterator, PartialEq, Eq, PartialOrd, Ord)]
pub enum Category {
    Asset,
    Liability,
    Equity,
    Income,
    Expenses,
}

impl Category {
    /// Return an iterator that iterates over all elements that are
    /// considered to be debit elements.
    ///
    /// These are Asset and Expenses.
    pub fn debits() -> DebitIter {
        DebitIter::new()
    }

    /// Return an iterator that iterates over all elements that are
    /// considered to be credit elements.
    ///
    /// These are Liability, Equity and Income.
    pub fn credits() -> CreditIter {
        CreditIter::new()
    }
}

/// Iterator over all debit categories.
pub struct DebitIter {
    debits: Vec<Category>,
}

/// Iterator over all credit categories.
pub struct CreditIter {
    credits: Vec<Category>,
}

impl DebitIter {
    fn new() -> Self {
        Self {
            debits: vec![Category::Asset, Category::Expenses],
        }
    }
}

impl CreditIter {
    fn new() -> Self {
        Self {
            credits: vec![
                Category::Liability,
                Category::Equity,
                Category::Income,
            ],
        }
    }
}

impl IntoIterator for DebitIter {
    type Item = Category;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.debits.into_iter()
    }
}

impl IntoIterator for CreditIter {
    type Item = Category;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.credits.into_iter()
    }
}

/// An account with a name and identifier
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Account {
    number: AccountNumber,
    name: AccountName,
    category: Category,
}

impl Account {
    pub fn new(number: AccountNumber, name: AccountName, element: Category) -> Self {
        Self { number, name, category: element }
    }

    pub fn number(&self) -> &AccountNumber {
        &self.number
    }

    pub fn name(&self) -> &AccountName {
        &self.name
    }

    pub fn category(&self) -> &Category {
        &self.category
    }
}

pub struct Chart {
    chart: BTreeMap<Category, BTreeSet<Account>>,
}

pub struct ChartIter<'a> {
    category_iter: std::collections::btree_map::Iter<'a, Category, BTreeSet<Account>>,
    account_iter: Option<std::collections::btree_set::Iter<'a, Account>>,
    current_element: Option<&'a Category>,
}

impl<'a> ChartIter<'a> {
    fn new(chart: &'a BTreeMap<Category, BTreeSet<Account>>) -> Self {
        Self {
            category_iter: chart.iter(),
            account_iter: None,
            current_element: None,
        }
    }
}

impl<'a> ChartIter<'a> {
    fn next_category(&mut self) -> Option<()> {
        if let Some((element, account)) = self.category_iter.next() {
            self.current_element = Some(element);
            self.account_iter = Some(account.iter());

            Some(())
        } else {
            None
        }
    }

    fn next_account(&mut self) -> Option<(&'a Category, &'a Account)> {
        if let Some(account) = self.account_iter.as_mut().unwrap().next() {
            Some((self.current_element.unwrap(), account))
        } else if let Some(()) = self.next_category() {
            self.next_account()
        } else {
            None
        }
    }
}

impl<'a> Iterator for ChartIter<'a> {
    type Item = (&'a Category, &'a Account);

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(_) = self.account_iter {
            self.next_account()
        } else {
            if let Some(()) = self.next_category() {
                self.next()
            } else {
                None
            }
        }
    }
}

impl Chart {
    pub fn new() -> Self {
        Self { chart: BTreeMap::new() }
    }

    pub fn push(&mut self, account: Account) {
        let element = account.category.clone();

        let en = self.chart.entry(element).or_insert(BTreeSet::new());
        en.insert(account);
    }

    pub fn iter(&self) -> ChartIter<'_> {
        ChartIter::new(&self.chart)
    }
}

/// This describes a "line" in a journal and notes one account being affected
/// with a debit or credit transaction.
pub struct JournalEntry {
    account: Account,
    transaction: Box<dyn TransactionMarker>,
}

impl JournalEntry {
    /// Returns a reference to the [Account] that is affected by this transaction
    pub fn account(&self) -> &Account {
        &self.account
    }

    /// Get the debit transaction for entry.
    ///
    /// If this is not a debit entry, None is returned, otherwise
    /// Some(&Transaction<Debit>>) is returned.
    pub fn debit(&self) -> Option<&Transaction<Debit>> {
        self.transaction.as_any().downcast_ref()
    }

    /// Get the credit transaction for entry.
    ///
    /// If this is not a credit entry, None is returned, otherwise
    /// Some(&Transaction<Credit>>) is returned.
    pub fn credit(&self) -> Option<&Transaction<Credit>> {
        self.transaction.as_any().downcast_ref()
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
pub struct Journal {
    details: EntryDetails,
    entries: Vec<JournalEntry>,
}

impl Journal {
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

    pub fn push(&mut self, entry: JournalEntry) {
        self.entries.push(entry);
    }

    pub fn as_slice(&self) -> &[JournalEntry] {
        self.entries.as_slice()
    }

    pub fn iter(&self) -> impl Iterator<Item = &JournalEntry> {
        self.entries.iter()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    use test_case::test_case;

    #[test_case("No leading" => Some(AccountName(String::from("No leading"))))]
    #[test_case("   Leading" => Some(AccountName(String::from("Leading"))))]
    #[test_case("Trailing\t" => Some(AccountName(String::from("Trailing"))))]
    #[test_case("\n Both \n" => Some(AccountName(String::from("Both"))))]
    #[test_case("\n  \n" => None)]
    fn account_name_new(input: &str) -> Option<AccountName> {
        AccountName::new(input)
    }

    #[test_case(Transaction::debit(50), Some(Transaction::debit(50)))]
    #[test_case(Transaction::credit(50), None)]
    fn journal_entry_debit<T>(tx: Transaction<T>, expected: Option<Transaction<Debit>>)
    where
        T: TransactionMarker,
    {
        let account = Account {
            name: AccountName(String::from("Test")),
            number: AccountNumber(54),
            category: Category::Asset,
        };

        let actual = JournalEntry {
            account,
            transaction: Box::new(tx),
        };

        assert_eq!(actual.debit(), expected.as_ref());
    }

    #[test_case(Transaction::credit(50), Some(Transaction::credit(50)))]
    #[test_case(Transaction::debit(50), None)]
    fn journal_entry_credit<T>(tx: Transaction<T>, expected: Option<Transaction<Credit>>)
    where
        T: TransactionMarker,
    {
        let account = Account {
            name: AccountName(String::from("Test")),
            number: AccountNumber(54),
            category: Category::Asset,
        };

        let actual = JournalEntry {
            account,
            transaction: Box::new(tx),
        };

        assert_eq!(actual.credit(), expected.as_ref());
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
            601.into(),
            AccountName(String::from("Grocery")),
            Category::Expenses,
        );

        chart.push(account.clone());

        let expected = vec![
            (&Category::Expenses, &account)
        ];

        let actual = chart
            .iter()
            .collect::<Vec<_>>();

        assert_eq!(actual, expected);
    }

    #[test]
    fn chart_iter_multiple() {
        let mut chart = Chart::new();

        let mut accounts = vec![
            Account::new(
                201.into(),
                AccountName::new("Credit Loan").unwrap(),
                Category::Liability,
            ),
            Account::new(
                401.into(),
                AccountName::new("Salary").unwrap(),
                Category::Income,
            ),
            Account::new(
                502.into(),
                AccountName::new("Phone").unwrap(),
                Category::Expenses,
            ),
            Account::new(
                501.into(),
                AccountName::new("Internet").unwrap(),
                Category::Expenses,
            ),
            Account::new(
                202.into(),
                AccountName::new("Bank Loan").unwrap(),
                Category::Liability,
            ),
            Account::new(
                101.into(),
                AccountName::new("Bank Account").unwrap(),
                Category::Asset
            ),
        ];

        for account in &accounts {
            chart.push(account.clone());
        }

        accounts.sort();
        let mut expected = Vec::new();
        for account in &accounts {
            expected.push((account.category(), account));
        }

        let actual = chart
            .iter()
            .collect::<Vec<_>>();

        assert_eq!(actual, expected);
    }
}
