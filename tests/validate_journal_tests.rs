use chrono::prelude::*;

use personal_finance::{
    balance::Transaction,
    entry::{Account, AccountName, Category, Journal},
};

#[test]
fn balanced_journal_should_be_valid() {
    let accounts: Vec<Account> = vec![
        Account::new(
            101,
            AccountName::new("Bank account").unwrap(),
            Category::Asset,
        ),
        Account::new(102, AccountName::new("Cash").unwrap(), Category::Asset),
        Account::new(
            501,
            AccountName::new("Groceries").unwrap(),
            Category::Expenses,
        ),
    ];

    let mut journal = Journal::new(Utc.ymd(2005, 4, 23));

    journal.push(&accounts[1], Transaction::credit(50));
    journal.push(&accounts[2], Transaction::debit(50));

    let journal = journal.validate();

    assert!(journal.is_ok());
}

#[test]
fn balanced_journal_should_be_invalid_given_non_zero_balance() {
    let accounts: Vec<Account> = vec![
        Account::new(
            101,
            AccountName::new("Bank account").unwrap(),
            Category::Asset,
        ),
        Account::new(102, AccountName::new("Cash").unwrap(), Category::Asset),
        Account::new(
            501,
            AccountName::new("Groceries").unwrap(),
            Category::Expenses,
        ),
    ];

    let mut journal = Journal::new(Utc.ymd(2005, 4, 23));

    journal.push(&accounts[1], Transaction::credit(50));
    journal.push(&accounts[2], Transaction::debit(52));

    let journal = journal.validate();

    assert!(journal.is_err());
}
