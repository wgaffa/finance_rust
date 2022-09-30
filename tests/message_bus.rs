use personal_finance::account::Category;
use tokio::{sync, task};

use cqrs::{error::AccountError, events::Balance};
use message_bus::{MailboxProcessor, Message};

#[tokio::test]
async fn create_account() {
    let mb = message_bus::MailboxProcessor::new().await;

    let (tx, mut rx) = sync::oneshot::channel();
    let result = mb
        .post(Message::CreateAccount {
            id: 101,
            description: String::from("Bank account"),
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
            id: 101,
            description: String::from("Bank account"),
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
            id: 101,
            description: String::from("Duplicate account"),
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
            transactions: vec![(101, Balance::Credit(150)), (501, Balance::Debit(150))],
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
                (101, Balance::Debit(10_000)),
                (401, Balance::Credit(10_000)),
            ],
            reply_channel: Some(tx),
        })
        .await;

    assert!(result.is_ok());

    let result = rx.await.unwrap();
    assert_eq!(result, Ok(2));
}
