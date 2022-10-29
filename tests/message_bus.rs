use std::{convert::TryInto, fmt::Debug};

use chrono::prelude::*;
use tokio::{sync, task};

use cqrs::{
    error::{AccountError, JournalError},
    events::store::InMemoryStore,
    write::ledger::LedgerId,
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

macro_rules! message {
    (open, $id:expr, $desc:expr, $cat:expr, $rc:expr) => {
        Message::CreateAccount {
            id: Number::new($id).unwrap(),
            description: Name::new($desc).unwrap(),
            category: $cat,
            reply_channel: $rc,
        }
    };

    (entry, $desc:expr, $date:expr => { $($account:expr => $ty:ident $amount:expr),* $(,)? }, $rc:expr) => {
        Message::JournalEntry {
            description: String::from($desc),
            transactions: vec![
                $(
                    (Number::new($account).unwrap(), Balance::$ty($amount).unwrap()),
                )*
            ],
            date: $date,
            reply_channel: $rc,
        }
    };

    (close, $acc:expr, $rc:expr) => {
        Message::CloseAccount { id: Number::new($acc).unwrap(), reply_channel: $rc }
    };

    (ledger, $name:expr, $rc:expr) => {
        Message::CreateLedger { id: LedgerId::new($name).unwrap(), reply_channel: $rc }
    };
}

macro_rules! message_with_reply {
    ($($tt:tt)*) => {
        {
            let (tx, rx) = sync::oneshot::channel();
            let m = message!($($tt)* , Some(tx));
            (m, rx)
        }
    };
}

#[tokio::test]
async fn create_account() {
    let mb = default_mailbox().await;

    let (message, mut rx) = message_with_reply!(open, 101, "Bank account", Category::Asset);
    let result = mb.post(message).await;

    let response = rx.await.unwrap();

    assert!(result.is_ok());
    assert!(response.is_ok());
}

#[tokio::test]
async fn create_duplicate_account() {
    let mb = default_mailbox().await;

    let (message, mut rx) = message_with_reply!(open, 101, "Bank account", Category::Asset);
    let result = mb.post(message).await;

    let response = rx.await.unwrap();

    assert!(result.is_ok());
    assert!(response.is_ok());

    let (message, mut rx) = message_with_reply!(open, 101, "Duplicate account", Category::Asset);
    let result = mb.post(message).await;

    let response = rx.await.unwrap();

    assert!(result.is_ok());
    assert_eq!(response, Err(AccountError::Opened(101)));
}

async fn add_default_account(mb: &MailboxProcessor) {
    let _ = mb
        .post(message!(open, 101, "Bank account", Category::Asset, None))
        .await;
    let _ = mb
        .post(message!(open, 501, "Groceries", Category::Expenses, None))
        .await;
    let _ = mb
        .post(message!(open, 401, "Salary", Category::Income, None))
        .await;
}

#[tokio::test]
async fn create_a_ledger_with_unique_id_should_succeed() {
    let mb = default_mailbox().await;
    let (message, rx) = message_with_reply!(ledger, "2014-q2");

    let result = mb.post(message).await;
    assert!(result.is_ok());

    let result = rx.await.unwrap();
    assert_eq!(result, Ok(()));
}

#[tokio::test]
async fn creating_a_ledger_with_same_id_should_be_an_error() {
    let mb = default_mailbox().await;
    let result = mb.post(message!(ledger, "2014-q2", None)).await;
    assert!(result.is_ok());

    let (message, rx) = message_with_reply!(ledger, "2014-q2");
    let result = mb.post(message).await;
    assert!(result.is_ok());

    let result = rx.await.unwrap();
    assert_eq!(result, Err(cqrs::error::LedgerError::AlreadyExists));
}

#[tokio::test]
async fn creating_an_entry_should_increase_its_id_counter() {
    let mb = default_mailbox().await;
    add_default_account(&mb).await;

    let (message, mut rx) = message_with_reply!(entry, "Grocery Shopping", Utc::now().date() => {
        101 => credit 150,
        501 => debit 150,
    });
    let result = mb.post(message).await;

    assert!(result.is_ok());

    let result = rx.await.unwrap();

    assert_eq!(result, Ok(1));

    let (message, mut rx) = message_with_reply!(entry, "Salary", Utc::now().date() => {
        101 => debit 10_000,
        401 => credit 10_000,
    });
    let result = mb.post(message).await;

    assert!(result.is_ok());

    let result = rx.await.unwrap();
    assert_eq!(result, Ok(2));
}

#[tokio::test]
async fn adding_a_transaction_to_a_non_existing_account_should_be_an_error() {
    let mb = default_mailbox().await;
    add_default_account(&mb).await;

    let (message, mut rx) = message_with_reply!(entry, "Grocery shopping", Utc::now().date() => {
        101 => credit 150,
        601 => debit 150,
    });
    let result = mb.post(message).await;

    assert!(result.is_ok());

    let response = rx.await.unwrap();
    assert_eq!(response, Err(JournalError::InvalidTransaction))
}

#[tokio::test]
async fn adding_no_transactions_to_an_entry_should_give_an_error() {
    let mb = default_mailbox().await;
    add_default_account(&mb).await;

    let (message, mut rx) = message_with_reply!(entry, "Grocery shopping", Utc::now().date() => {
        // empty transactions
    });
    let result = mb.post(message).await;

    assert!(result.is_ok());

    let response = rx.await.unwrap();
    assert_eq!(response, Err(JournalError::EmptyTransaction))
}

#[tokio::test]
async fn closing_an_account_twice_should_give_an_error() {
    let mb = default_mailbox().await;
    add_default_account(&mb).await;

    let (message, mut rx) = message_with_reply!(close, 101);
    let result = mb.post(message).await;

    assert!(result.is_ok());

    let response = rx.await.unwrap();
    assert_eq!(response, Ok(()));

    let (message, mut rx) = message_with_reply!(close, 101);
    let result = mb.post(message).await;

    assert!(result.is_ok());

    let response = rx.await.unwrap();
    assert_eq!(response, Err(AccountError::Closed));
}

#[tokio::test]
async fn closing_a_non_existent_account_should_give_an_error() {
    let mb = default_mailbox().await;

    let (message, mut rx) = message_with_reply!(close, 101);
    let result = mb.post(message).await;
    let response = rx.await.unwrap();
    assert_eq!(response, Err(AccountError::Closed));
}
