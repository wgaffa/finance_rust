use std::{convert::TryInto, fmt::Debug};

use chrono::prelude::*;
use tokio::{sync, task};

use cqrs::{
    error::{AccountError, JournalError},
    events::store::InMemoryStore,
};
use message_bus::{CommandHandler, MailboxProcessor, Message};
use personal_finance::{
    account::{Category, Name, Number},
    balance::Balance,
};

async fn default_mailbox() -> MailboxProcessor {
    let handler = CommandHandler::new(InMemoryStore::default());
    MailboxProcessor::new(handler).await
}

macro_rules! post_message {
    (open $t:ident , $id:expr, $desc:expr, $cat:expr, $rc:expr) => {
        $t.post(Message::CreateAccount {
            id: Number::new($id).unwrap(),
            description: Name::new($desc).unwrap(),
            category: $cat,
            reply_channel: $rc,
        })
        .await
    };

    (entry $t:ident , $desc:expr, $date:expr, $rc:expr => { $($account:expr => $ty:ident $amount:expr),* $(,)? }) => {
        $t.post(Message::JournalEntry {
            description: String::from($desc),
            transactions: vec![
                $(
                    (Number::new($account).unwrap(), Balance::$ty($amount).unwrap()),
                )*
            ],
            date: $date,
            reply_channel: $rc,
        })
        .await
    };

    (close $t:ident , $acc:expr, $rc:expr) => {
        $t.post(Message::CloseAccount { id: Number::new($acc).unwrap(), reply_channel: $rc }).await
    };
}

#[tokio::test]
async fn create_account() {
    let mb = default_mailbox().await;

    let (tx, mut rx) = sync::oneshot::channel();
    let result = post_message!(open mb, 101, "Bank account", Category::Asset, Some(tx));

    let response = rx.await.unwrap();

    assert!(result.is_ok());
    assert!(response.is_ok());
}

#[tokio::test]
async fn create_duplicate_account() {
    let mb = default_mailbox().await;

    let (tx, mut rx) = sync::oneshot::channel();

    let result = post_message!(open mb, 101, "Bank account", Category::Asset, Some(tx));

    let response = rx.await.unwrap();

    assert!(result.is_ok());
    assert!(response.is_ok());

    let (tx, mut rx) = sync::oneshot::channel();
    let result = post_message!(open mb, 101, "Duplicate account", Category::Asset, Some(tx) );

    let response = rx.await.unwrap();

    assert!(result.is_ok());
    assert_eq!(response, Err(AccountError::AccountAlreadyOpened(101)));
}

async fn add_default_account(mb: &MailboxProcessor) {
    let _ = post_message!(open mb, 101, "Bank account", Category::Asset, None );
    let _ = post_message!(open mb, 501, "Groceries", Category::Expenses, None );
    let _ = post_message!(open mb, 401, "Salary", Category::Income, None );
}

#[tokio::test]
async fn create_journal() {
    let mb = default_mailbox().await;
    add_default_account(&mb).await;

    let (tx, mut rx) = sync::oneshot::channel();
    let result = post_message!(entry mb, "Grocery shopping", Utc::now().date(), Some(tx) => {
        101 => credit 150,
        501 => debit 150,
    });

    assert!(result.is_ok());

    let result = rx.await.unwrap();

    assert_eq!(result, Ok(1));

    let (tx, mut rx) = sync::oneshot::channel();
    let result = post_message!(entry mb, "Salary", Utc::now().date(), Some(tx) => {
        101 => debit 10_000,
        401 => credit 10_000,
    });

    assert!(result.is_ok());

    let result = rx.await.unwrap();
    assert_eq!(result, Ok(2));
}

#[tokio::test]
async fn adding_a_transaction_to_a_non_existing_account_should_be_an_error() {
    let mb = default_mailbox().await;
    add_default_account(&mb).await;

    let (tx, mut rx) = sync::oneshot::channel();
    let result = post_message!(entry mb, "Grocery shopping", Utc::now().date(), Some(tx) => {
        101 => credit 150,
        601 => debit 150,
    });

    assert!(result.is_ok());

    let response = rx.await.unwrap();
    assert_eq!(response, Err(JournalError::InvalidTransaction))
}

#[tokio::test]
async fn adding_no_transactions_to_an_entry_should_give_an_error() {
    let mb = default_mailbox().await;
    add_default_account(&mb).await;

    let (tx, mut rx) = sync::oneshot::channel();
    let result = post_message!(entry mb, "Grocery shopping", Utc::now().date(), Some(tx) => {
        // empty transactions
    });

    assert!(result.is_ok());

    let response = rx.await.unwrap();
    assert_eq!(response, Err(JournalError::EmptyTransaction))
}

#[tokio::test]
async fn closing_an_account_twice_should_give_an_error() {
    let mb = default_mailbox().await;
    add_default_account(&mb).await;

    let (tx, mut rx) = sync::oneshot::channel();
    let result = post_message!(close mb, 101, Some(tx));

    assert!(result.is_ok());

    let response = rx.await.unwrap();
    assert_eq!(response, Ok(()));

    let (tx, mut rx) = sync::oneshot::channel();
    let result = post_message!(close mb, 101, Some(tx));

    assert!(result.is_ok());

    let response = rx.await.unwrap();
    assert_eq!(response, Err(AccountError::AccountAlreadyClosed));
}
