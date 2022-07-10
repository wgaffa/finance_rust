use chrono::prelude::*;

use personal_finance::{
    account::{self, Category},
    balance::Transaction,
    entry::{Account, Journal},
};

pub fn accounts() -> Vec<Account> {
    vec![
        Account::new(
            101,
            account::Name::new("Bank account").unwrap(),
            Category::Asset,
        ),
        Account::new(102, account::Name::new("Cash").unwrap(), Category::Asset),
        Account::new(
            501,
            account::Name::new("Groceries").unwrap(),
            Category::Expenses,
        ),
    ]
}

#[test]
fn balanced_journal_should_be_valid() {
    let accounts = accounts();

    let mut journal = Journal::new(Utc.ymd(2005, 4, 23));

    journal.push(&accounts[1], Transaction::credit(50).unwrap());
    journal.push(&accounts[2], Transaction::debit(50).unwrap());

    let expected = journal.clone();

    let journal = journal.validate();

    assert!(journal.is_ok());
    assert_eq!(journal.unwrap(), expected);
}

#[test]
fn balanced_journal_should_be_valid_given_split_transaction() {
    let accounts = accounts();

    let mut journal = Journal::new(Utc.ymd(2005, 4, 23));

    journal.push(&accounts[1], Transaction::credit(50).unwrap());
    journal.push(&accounts[2], Transaction::debit(10).unwrap());
    journal.push(&accounts[2], Transaction::debit(30).unwrap());
    journal.push(&accounts[2], Transaction::debit(10).unwrap());

    let expected = journal.clone();

    let journal = journal.validate();

    assert!(journal.is_ok());
    assert_eq!(journal.unwrap(), expected);
}

#[test]
fn balanced_journal_should_be_invalid_given_non_zero_balance() {
    let accounts = accounts();

    let mut journal = Journal::new(Utc.ymd(2005, 4, 23));

    journal.push(&accounts[1], Transaction::credit(50).unwrap());
    journal.push(&accounts[2], Transaction::debit(52).unwrap());

    let journal = journal.validate();

    assert!(journal.is_err());
}
