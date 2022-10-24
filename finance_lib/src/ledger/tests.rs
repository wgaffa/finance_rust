use chrono::prelude::*;

use super::*;

use crate::{
    account::{self, Category},
    balance::Balance,
    entry::Account,
};

#[test]
fn ledger_iter() {
    let account = Account::new(account::Number::new(101).unwrap(), account::Name::new("test").unwrap(), Category::Asset);
    let mut ledger = Ledger::new(&account);

    let transactions = vec![
        Balance::debit(150).unwrap(),
        Balance::debit(270).unwrap(),
        Balance::credit(50).unwrap(),
    ];
    let entries = vec![
        (Utc.ymd(2021, 2, 10), transactions[0]),
        (Utc.ymd(2021, 2, 15), transactions[1]),
        (Utc.ymd(2021, 3, 5), transactions[2]),
    ];

    for entry in &entries {
        ledger.entries.push(LedgerEntry {
            date: entry.0,
            transaction: entry.1.to_owned(),
        });
    }

    let actual = ledger.iter().collect::<Vec<_>>();

    let expected = entries.iter().map(|(d, b)| (d, b)).collect::<Vec<_>>();

    assert_eq!(actual, expected);
}
