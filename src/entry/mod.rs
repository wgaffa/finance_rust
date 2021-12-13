use chrono::prelude::*;

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

#[derive(Debug)]
enum AccountElement {
    Asset,
    Liability,
    Equity,
    Income,
    Expenses,
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

#[cfg(test)]
mod test {
    use super::*;

    use test_case::test_case;

    use crate::bookkeeping::Transaction;

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
    	let account = Account {
        	name: AccountName::new("Test").unwrap(),
        	number: AccountNumber(54),
        	element: AccountElement::Income,
    	};

    	let _actual = JournalEntry {
        	account,
        	transaction: Box::new(Transaction::debit(50)),
    	};
	}
}
