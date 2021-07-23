#[cfg(test)]
use super::*;

use test_case::test_case;
use std::any::{Any, TypeId};

fn is_debit_transaction<T: ?Sized + Any>(_t : &T) -> bool {
    TypeId::of::<Transaction<Debit>>() == TypeId::of::<T>()
}

fn is_credit_transaction<T: ?Sized + Any>(_t : &T) -> bool {
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

