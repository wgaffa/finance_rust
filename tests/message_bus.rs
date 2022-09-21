use message_bus::{MailboxProcessor, Message};
use tokio::{task, sync};

#[tokio::test]
async fn create_account() {
    let mb = message_bus::MailboxProcessor::new().await;

    let (tx, mut rx) = sync::oneshot::channel();
    let result = mb
        .post(Message::CreateAccount {
            id: 101,
            description: String::from("Bank account"),
            reply_channel: Some(tx),
        })
        .await;

    let response = rx.await;

    assert!(result.is_ok());
    assert!(response.is_ok());
}
