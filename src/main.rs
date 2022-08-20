use cqrs::events::{
    store::{EventStorage, InMemoryStore},
    Balance,
    Event,
};
use personal_finance::{self, account::Category};

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
    store.append(Event::Transaction {
        account: 101,
        amount: Balance::Debit(50),
        journal: 1,
    });
    store.append(Event::Transaction {
        account: 101,
        amount: Balance::Debit(15),
        journal: 1,
    });
    store.append(Event::Transaction {
        account: 101,
        amount: Balance::Credit(25),
        journal: 1,
    });

    for event in store.iter() {
        println!("{event:#?}");
    }
}
