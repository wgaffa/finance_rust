use chrono::prelude::*;

use super::*;

use crate::{
    balance::{Balance, Transaction, TransactionMarker},
    entry::{Account, AccountName, Category},
};

#[test]
fn ledger_iter() {
    let account = Account::new(
        101.into(),
        AccountName::new("test").unwrap(),
        Category::Asset,
    );
    let mut ledger = Ledger::new(&account);

    let transactions: Vec<Box<dyn TransactionMarker>> = vec![
        Box::new(Transaction::debit(150)),
        Box::new(Transaction::debit(270)),
        Box::new(Transaction::credit(50)),
    ];
    let entries = vec![
        (
            Utc.ymd(2021, 2, 10),
            Balance::Debit(transactions[0].as_debit().unwrap().to_owned()),
        ),
        (
            Utc.ymd(2021, 2, 15),
            Balance::Debit(transactions[1].as_debit().unwrap().to_owned()),
        ),
        (
            Utc.ymd(2021, 3, 5),
            Balance::Credit(transactions[2].as_credit().unwrap().to_owned()),
        ),
    ];

    for entry in &entries {
        ledger.entries.push(LedgerEntry {
            date: entry.0,
            transaction: entry.1.to_owned(),
        });
    }

    let actual = ledger.iter().cloned().collect::<Vec<_>>();

    let expected = entries
        .into_iter()
        .map(|(date, transaction)| LedgerEntry { date, transaction })
        .collect::<Vec<_>>();

    assert_eq!(actual, expected);
}
