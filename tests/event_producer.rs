use cqrs::{
    behaviour,
    events::{
        store::{EventStorage, InMemoryStore},
        Event,
    },
};
use personal_finance::account::{Category, Name};

#[test]
fn create_new_account_in_empty_chart() {
    let add_event: Box<dyn Fn(&[Event]) -> Vec<Event>> =
        Box::new(|_events| {
            vec![Event::AccountOpened {
                id: 101,
                name: String::from("Bank Account"),
                category: Category::Asset,
            }]
        });
    let mut repo = InMemoryStore::new();

    repo.evolve(add_event).unwrap();
    let current_events = repo.iter().cloned().collect::<Vec<_>>();

    let expected = vec![Event::AccountOpened {
        id: 101,
        name: String::from("Bank Account"),
        category: Category::Asset,
    }];

    assert_eq!(current_events, expected);
}

#[test]
fn creating_account() {
    let mut repo = InMemoryStore::new();

    repo.evolve(|e| {
        behaviour::open_account(101, String::from("Credit Account"), Category::Asset, e)
    })
    .unwrap();
    repo.evolve(|e| behaviour::open_account(201, String::from("Groceries"), Category::Expenses, e))
        .unwrap();
    repo.evolve(|e| behaviour::open_account(301, String::from("Salary"), Category::Income, e))
        .unwrap();

    let actual = repo.iter().cloned().collect::<Vec<_>>();

    let expected = vec![
        Event::AccountOpened {
            id: 101,
            name: String::from("Credit Account"),
            category: Category::Asset,
        },
        Event::AccountOpened {
            id: 201,
            name: String::from("Groceries"),
            category: Category::Expenses,
        },
        Event::AccountOpened {
            id: 301,
            name: String::from("Salary"),
            category: Category::Income,
        },
    ];

    assert_eq!(actual, expected);
}

#[test]
fn creating_duplicate_should_give_error() {
    let mut repo = InMemoryStore::new();

    let mut write_model = cqrs::Chart::default();
    let new_events = write_model.open(101.into(), Name::new("Credit Account").unwrap(), Category::Asset).unwrap();
    repo.extend(new_events.iter().cloned());

    let res = write_model.open(101.into(), Name::new("Bank Account").unwrap(), Category::Asset);

    let actual = repo.into_iter().collect::<Vec<_>>();
    let expected = vec![Event::AccountOpened {
        id: 101,
        name: String::from("Credit Account"),
        category: Category::Asset,
    }];

    assert_eq!(actual, expected);
    assert!(res.is_err());
}
