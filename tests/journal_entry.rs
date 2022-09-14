use cqrs::events::{Event, Balance, store::InMemoryStore};
use personal_finance::account::Category;

#[test]
fn simple_journal_entry() {
    let event_history = [
        Event::AccountOpened { id: 101, name: String::from("Bank account"), category: Category::Asset },
        Event::AccountOpened { id: 501, name: String::from("Groceries"), category: Category::Expenses },
    ];

    let mut journal = cqrs::Journal::new(&event_history);
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
