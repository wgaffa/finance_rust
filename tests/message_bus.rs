use chrono::prelude::*;
use tokio::{sync, task};

use cqrs::error::AccountError;
use message_bus::{MailboxProcessor, Message};
use personal_finance::{
    account::{Category, Name, Number},
    balance::Balance,
};

#[tokio::test]
async fn create_account() {
    let mb = message_bus::MailboxProcessor::new().await;

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
    let mb = message_bus::MailboxProcessor::new().await;

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

#[tokio::test]
async fn create_journal() {
    let mb = message_bus::MailboxProcessor::new().await;

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
