use chrono::prelude::*;

use cqrs::{events::{Event, store::InMemoryStore}, error::JournalError};
use personal_finance::{balance::Balance, account::{Name, Number, Category}};

fn events() -> [Event; 2] {
    [
        Event::AccountOpened { id: Number::new(101).unwrap(), name: Name::new("Bank account").unwrap(), category: Category::Asset },
        Event::AccountOpened { id: Number::new(501).unwrap(), name: Name::new("Groceries").unwrap(), category: Category::Expenses },
    ]
}

#[test]
fn simple_journal_entry() {
    let mut journal = cqrs::Journal::new(&events());
    let entry = journal.entry("Starting Balance", &[
        (Number::new(101).unwrap(), Balance::credit(50).unwrap()),
        (Number::new(501).unwrap(), Balance::debit(50).unwrap()),
    ],
    Utc.ymd(2014, 5, 23));

    assert!(entry.is_ok());

    let entry = entry.unwrap();
    let expected = vec![
        Event::Journal { id: 1, description: String::from("Starting Balance"), date: Utc.ymd(2014, 5, 23) },
        Event::Transaction { account: Number::new(101).unwrap(), amount: Balance::credit(50).unwrap(), journal: 1 },
        Event::Transaction { account: Number::new(501).unwrap(), amount: Balance::debit(50).unwrap(), journal: 1 },
    ];

    assert_eq!(expected, entry);
}

#[test]
fn imbalanced_journal_entry() {
    let mut journal = cqrs::Journal::new(&events());
    let entry = journal.entry("Starting Balance", &[
        (Number::new(101).unwrap(), Balance::credit(50).unwrap()),
        (Number::new(501).unwrap(), Balance::debit(50).unwrap()),
        (Number::new(101).unwrap(), Balance::debit(10).unwrap()),
    ],
    Utc.ymd(2014, 5, 23));

    assert!(entry.is_err());
    assert_eq!(entry, Err(JournalError::ImbalancedTranasactions));
}

#[test]
fn empty_transactions_entry_should_be_invalid() {
    let mut journal = cqrs::Journal::new(&events());
    let entry = journal.entry("Starting Balance", &[], Utc::now().date());

    assert!(entry.is_err());
    assert_eq!(entry, Err(JournalError::EmptyTransaction));
}
