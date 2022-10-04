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

#[tokio::test]
async fn create_account() {
    let mb = default_mailbox().await;

    let (tx, mut rx) = sync::oneshot::channel();
    let result = mb
        .post(Message::CreateAccount {
            id: Number::new(101).unwrap(),
            description: Name::new("Bank account").unwrap(),
            category: Category::Asset,
            reply_channel: Some(tx),
        })
        .await;

    let response = rx.await.unwrap();

    assert!(result.is_ok());
    assert!(response.is_ok());
}

#[tokio::test]
async fn create_duplicate_account() {
    let mb = default_mailbox().await;

    let (tx, mut rx) = sync::oneshot::channel();

    let result = mb
        .post(Message::CreateAccount {
            id: Number::new(101).unwrap(),
            description: Name::new("Bank account").unwrap(),
            category: Category::Asset,
            reply_channel: Some(tx),
        })
        .await;

    let response = rx.await.unwrap();

    assert!(result.is_ok());
    assert!(response.is_ok());

    let (tx, mut rx) = sync::oneshot::channel();
    let result = mb
        .post(Message::CreateAccount {
            id: Number::new(101).unwrap(),
            description: Name::new("Duplicate account").unwrap(),
            category: Category::Asset,
            reply_channel: Some(tx),
        })
        .await;

    let response = rx.await.unwrap();

    assert!(result.is_ok());
    assert_eq!(response, Err(AccountError::AccountAlreadyOpened(101)));
}

async fn add_default_account(mb: &MailboxProcessor) {
    let _ = mb
        .post(Message::CreateAccount {
            id: Number::new(101).unwrap(),
            description: Name::new("Bank account").unwrap(),
            category: Category::Asset,
            reply_channel: None,
        })
        .await;
    let _ = mb
        .post(Message::CreateAccount {
            id: Number::new(501).unwrap(),
            description: Name::new("Groceries").unwrap(),
            category: Category::Expenses,
            reply_channel: None,
        })
        .await;
    let _ = mb
        .post(Message::CreateAccount {
            id: Number::new(401).unwrap(),
            description: Name::new("Salary").unwrap(),
            category: Category::Income,
            reply_channel: None,
        })
        .await;
}

#[tokio::test]
async fn create_journal() {
    let mb = default_mailbox().await;
    add_default_account(&mb).await;

    let (tx, mut rx) = sync::oneshot::channel();
    let result = mb
        .post(Message::JournalEntry {
            description: String::from("Grocery shopping"),
            transactions: vec![
                (Number::new(101).unwrap(), Balance::credit(150).unwrap()),
                (Number::new(501).unwrap(), Balance::debit(150).unwrap()),
            ],
            date: Utc::now().date(),
            reply_channel: Some(tx),
        })
        .await;

    assert!(result.is_ok());

    let result = rx.await.unwrap();

    assert_eq!(result, Ok(1));

    let (tx, mut rx) = sync::oneshot::channel();
    let result = mb
        .post(Message::JournalEntry {
            description: String::from("Salary"),
            transactions: vec![
                (Number::new(101).unwrap(), Balance::debit(10_000).unwrap()),
                (Number::new(401).unwrap(), Balance::credit(10_000).unwrap()),
            ],
            date: Utc::now().date(),
            reply_channel: Some(tx),
        })
        .await;

    assert!(result.is_ok());

    let result = rx.await.unwrap();
    assert_eq!(result, Ok(2));
}

#[tokio::test]
async fn adding_a_transaction_to_a_non_existing_account_should_be_an_error() {
    let mb = default_mailbox().await;
    add_default_account(&mb).await;

    let (tx, mut rx) = sync::oneshot::channel();
    let result = mb
        .post(Message::JournalEntry {
            description: String::from("Grocery shopping"),
            transactions: vec![
                (Number::new(101).unwrap(), Balance::credit(150).unwrap()),
                (Number::new(601).unwrap(), Balance::debit(150).unwrap()),
            ],
            date: Utc::now().date(),
            reply_channel: Some(tx),
        })
        .await;

    assert!(result.is_ok());

    let response = rx.await.unwrap();
    assert_eq!(response, Err(JournalError::InvalidTransaction))
}

#[tokio::test]
async fn adding_no_transactions_to_an_entry_should_give_an_error() {
    let mb = default_mailbox().await;
    add_default_account(&mb).await;

    let (tx, mut rx) = sync::oneshot::channel();
    let result = mb
        .post(Message::JournalEntry {
            description: String::from("Grocery shopping"),
            transactions: vec![],
            date: Utc::now().date(),
            reply_channel: Some(tx),
        })
        .await;

    assert!(result.is_ok());

    let response = rx.await.unwrap();
    assert_eq!(response, Err(JournalError::EmptyTransaction))
}
