use cqrs::{events::{Event, Balance, store::InMemoryStore}, error::JournalError};
use personal_finance::account::Category;

fn events() -> [Event; 2] {
    [
        Event::AccountOpened { id: 101, name: String::from("Bank account"), category: Category::Asset },
        Event::AccountOpened { id: 501, name: String::from("Groceries"), category: Category::Expenses },
    ]
}

#[test]
fn simple_journal_entry() {
    let mut journal = cqrs::Journal::new(&events());
    let entry = journal.entry("Starting Balance", &[
        (101, Balance::Credit(50)),
        (501, Balance::Debit(50)),
    ]);

    assert!(entry.is_ok());

    let entry = entry.unwrap();
    let expected = vec![
        Event::Journal { id: 1, description: String::from("Starting Balance") },
        Event::Transaction { account: 101, amount: Balance::Credit(50), journal: 1 },
        Event::Transaction { account: 501, amount: Balance::Debit(50), journal: 1 },
    ];

    assert_eq!(expected, entry);
}

#[test]
fn imbalanced_journal_entry() {
    let mut journal = cqrs::Journal::new(&events());
    let entry = journal.entry("Starting Balance", &[
        (101, Balance::Credit(50)),
        (501, Balance::Debit(50)),
        (101, Balance::Debit(10)),
    ]);

    assert!(entry.is_err());
    assert_eq!(entry, Err(JournalError::ImbalancedTranasactions));
}

#[test]
fn empty_transactions_entry_should_be_invalid() {
    let mut journal = cqrs::Journal::new(&events());
    let entry = journal.entry("Starting Balance", &[]);

    assert!(entry.is_err());
    assert_eq!(entry, Err(JournalError::EmptyTransaction));
}
