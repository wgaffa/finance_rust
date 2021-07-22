#[cfg(test)]
use super::*;

use test_case::test_case;

#[test]
fn new_debit_test() {
    let actual = Transaction::debit(Amount::new(500));

    assert_eq!(actual.entry, Entry::Debit);
    assert_eq!(actual.amount.0, 500);
}

#[test_case(100, 100)]
#[test_case(-300, 300)]
#[test_case(i32::MIN, 5 => panics "attempt to negate with overflow")]
fn new_credit_test(amount: i32, expected: i32) {
    let actual = Transaction::credit(Amount::new(amount));

    assert_eq!(actual.entry, Entry::Credit);
    assert_eq!(actual.amount.0, expected);
}

#[test_case(Amount::new(100), 100)]
#[test_case(Amount::new(i32::MAX), i32::MAX)]
#[test_case(Amount::new(-200), 200)]
fn debit_to_numeral(amount: AmountType, expected: i32) {
    let transaction = Transaction::debit(amount);

    let actual = transaction.to_numeral();

    assert_eq!(actual, expected);
}

#[test_case(Amount::new(100), -100)]
#[test_case(Amount::new(i32::MAX), -i32::MAX)]
#[test_case(Amount::new(-200), -200)]
fn credit_to_numeral(amount: AmountType, expected: i32) {
    let transaction = Transaction::credit(amount);

    let actual = transaction.to_numeral();

    assert_eq!(actual, expected);
}
