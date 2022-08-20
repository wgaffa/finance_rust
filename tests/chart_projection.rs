use cqrs::events::{
    projections::Projection,
    store::{EventStorage, InMemoryStore},
    Event,
};
use personal_finance::{
    account::{Category, Name},
    entry::{Account, Chart},
};

#[test]
fn chart_projection() {
    let mut repository = InMemoryStore::new();
    repository.append(Event::AccountOpened {
        id: 101,
        name: String::from("Cash Account"),
        category: Category::Asset,
    });
    repository.append(Event::AccountOpened {
        id: 102,
        name: String::from("Savings Account"),
        category: Category::Asset,
    });
    repository.append(Event::AccountOpened {
        id: 501,
        name: String::from("Groceries"),
        category: Category::Expenses,
    });

    let projection = Projection::new(Chart::new(), |mut state, event| {
        if let Event::AccountOpened { id, name, category } = event {
            state.insert(Account::new(*id, Name::new(name).unwrap(), *category));
        }

        state
    });

    let chart = projection.project(repository.iter());
    let actual = chart.iter().cloned().collect::<Vec<_>>();

    let expected = vec![
        Account::new(101, Name::new("Cash Account").unwrap(), Category::Asset),
        Account::new(102, Name::new("Savings Account").unwrap(), Category::Asset),
        Account::new(501, Name::new("Groceries").unwrap(), Category::Expenses),
    ];

    assert_eq!(actual, expected);
}
