#[cfg(test)]
use super::*;

use std::any::{Any, TypeId};
use test_case::test_case;

fn is_debit_transaction<T: ?Sized + Any>(_t: &T) -> bool {
    TypeId::of::<Transaction<Debit>>() == TypeId::of::<T>()
}

fn is_credit_transaction<T: ?Sized + Any>(_t: &T) -> bool {
    TypeId::of::<Transaction<Credit>>() == TypeId::of::<T>()
}

#[test_case(100, 100)]
#[test_case(u32::MAX, 4294967295)]
fn new_debit_test(amount: u32, expected: u32) {
    let actual = Transaction::debit(amount);

    assert!(is_debit_transaction(&actual));
    assert_eq!(actual.amount, expected);
}

#[test_case(100, 100)]
#[test_case(u32::MAX, 4294967295)]
fn new_credit_test(amount: u32, expected: u32) {
    let actual = Transaction::credit(amount);

    assert!(is_credit_transaction(&actual));
    assert_eq!(actual.amount, expected);
}

#[test_case(50, |x| x * 2 => 100)]
#[test_case(u32::MAX, |x| x + 1 => panics "overflow")]
fn transaction_debit_map<F: Fn(u32) -> u32>(amount: u32, f: F) -> u32 {
    let actual = Transaction::debit(amount);

    let actual = actual.map(f);

    actual.amount()
}

#[test_case(50, |x| x * 2 => 100)]
#[test_case(u32::MAX, |x| x + 1 => panics "overflow")]
fn transaction_credit_map<F: Fn(u32) -> u32>(amount: u32, f: F) -> u32 {
    let actual = Transaction::credit(amount);

    let actual = actual.map(f);

    actual.amount()
}

#[test]
fn sum_trait_iter() {
    let vec = vec![
        Transaction::debit(50),
        Transaction::debit(20),
        Transaction::debit(30),
    ];

    let actual: Transaction<Debit> = vec.iter().sum();

    assert_eq!(actual.amount, 100);
}

#[test]
fn sum_trait_into_iter() {
    let vec = vec![
        Transaction::debit(50),
        Transaction::debit(20),
        Transaction::debit(30),
    ];

    let actual: Transaction<Debit> = vec.into_iter().sum();

    assert_eq!(actual.amount, 100);
}

#[test]
fn split_transactions() {
    let vec: Vec<Box<dyn TransactionMarker>> = vec![
        Box::new(Transaction::debit(50)),
        Box::new(Transaction::credit(20)),
        Box::new(Transaction::debit(50)),
    ];

    let (debits, credits) = split(vec);

    let debit_sum = debits.into_iter().sum::<Transaction<Debit>>();
    let credit_sum = credits.into_iter().sum::<Transaction<Credit>>();

    assert_eq!(debit_sum.amount, 100);
    assert_eq!(credit_sum.amount, 20);
}

struct MockTest;

impl TransactionMarker for MockTest {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[test]
#[should_panic(expected = "incompatible types")]
fn split_transaction_panics_on_wrong_types() {
    let vec: Vec<Box<dyn TransactionMarker>> = vec![
        Box::new(Transaction::debit(50)),
        Box::new(Transaction::credit(20)),
        Box::new(MockTest),
    ];

    let (_debits, _credits) = split(vec);
}

#[test]
fn to_balance_should_return_debit_balance_given_transaction_debit() {
    let debit = Transaction::debit(50);
    let actual = debit.into_balance();

    let expected = Balance::Debit(Transaction::debit(50));
    assert_eq!(actual, expected);
}

#[test]
fn to_balance_should_return_credit_balance_given_transaction_credit() {
    let credit = Transaction::credit(50);
    let actual = credit.into_balance();

    let expected = Balance::Credit(Transaction::credit(50));
    assert_eq!(actual, expected);
}
