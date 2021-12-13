use chrono::prelude::*;
use enum_iterator::IntoEnumIterator;

use crate::bookkeeping::TransactionMarker;

#[derive(Debug)]
pub struct EntryDetails {
    date: Date<Utc>,
    description: Option<String>,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct AccountNumber(u32);

impl AccountNumber {
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

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct AccountName(String);

impl AccountName {
    pub fn new<T: AsRef<str>>(name: T) -> Option<Self> {
        let name = name.as_ref().trim().to_owned();
        if name.is_empty() {
            None
        } else {
            Some(AccountName(name))
        }
    }
}

#[derive(Debug, Clone, IntoEnumIterator)]
pub enum AccountElement {
    Asset,
    Liability,
    Equity,
    Income,
    Expenses,
}

impl AccountElement {
    pub fn debits() -> DebitIter {
        DebitIter::new()
    }

    pub fn credits() -> CreditIter {
        CreditIter::new()
    }
}

pub struct DebitIter {
    debits: Vec<AccountElement>
}

pub struct CreditIter {
    credits: Vec<AccountElement>
}

impl DebitIter {
    fn new() -> Self {
        Self {
            debits: vec![AccountElement::Asset, AccountElement::Expenses],
        }
    }
}

impl CreditIter {
    fn new() -> Self {
        Self {
            credits: vec![
                AccountElement::Liability,
                AccountElement::Equity,
                AccountElement::Income,
            ],
        }
    }
}

impl IntoIterator for DebitIter {
    type Item = AccountElement;
    type IntoIter = std::vec::IntoIter<Self::Item>;

	fn into_iter(self) -> Self::IntoIter {
    	self.debits.into_iter()
	}
}

impl IntoIterator for CreditIter {
    type Item = AccountElement;
    type IntoIter = std::vec::IntoIter<Self::Item>;

	fn into_iter(self) -> Self::IntoIter {
    	self.credits.into_iter()
	}
}

#[derive(Debug)]
pub struct Account {
    number: AccountNumber,
    name: AccountName,
    element: AccountElement,
}

pub struct JournalEntry {
    account: Account,
    transaction: Box<dyn TransactionMarker>,
}

pub struct Journal {
    details: EntryDetails,
    entries: Vec<JournalEntry>,
}

impl Journal {
    pub fn new(date: Date<Utc>) -> Self {
        Self {
            details: EntryDetails {
                date,
                description: None
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

    pub fn push(&mut self, entry: JournalEntry) {
        self.entries.push(entry);
    }

    pub fn as_slice(&self) -> &[JournalEntry] {
        &self.entries.as_slice()
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

	#[test]
	fn simple() {
    	// Use this space to experiment with some ideas
	}
}
