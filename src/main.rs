use cqrs::events::{
    store::{EventStorage, InMemoryStore},
    Event,
};
use personal_finance::{self, account::Category, balance::Balance};

fn main() {
    let mut store = InMemoryStore::new();

    store.append(Event::AccountOpened {
        id: 101,
        name: String::from("Bank account"),
        category: Category::Asset,
    });
    store.append(Event::AccountOpened {
        id: 501,
        name: String::from("General expenses"),
        category: Category::Expenses,
    });
    store.append(Event::Transaction { account: 101, amount: Balance::debit(50).unwrap(), journal: 1 });
    store.append(Event::Transaction { account: 101, amount: Balance::debit(15).unwrap(), journal: 1 });
    store.append(Event::Transaction { account: 101, amount: Balance::credit(25).unwrap(), journal: 1 });

    for event in store.iter() {
        println!("{event:#?}");
    }
}
