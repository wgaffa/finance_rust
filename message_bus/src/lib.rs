use error_stack::{Result, ResultExt, IntoReport};
use tokio::{
    sync::{self, mpsc::{self, Sender}},
    task,
};

#[derive(Debug)]
pub enum MailboxProcessorError {
    SenderChannelClosed,
}

impl std::fmt::Display for MailboxProcessorError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Self::SenderChannelClosed => f.write_str("Could not send due to channel is terminated"),
        }
    }
}

impl std::error::Error for MailboxProcessorError {}

pub struct MailboxProcessor {
    sender: Sender<Message>,
}

impl MailboxProcessor {
    pub async fn new() -> Self {
        let (sender, mut receiver) = mpsc::channel(32);

        task::spawn(async move {
            loop {
                match receiver.recv().await {
                    None => break,
                    Some(message) => process_message(message).await,
                }
            }
        });

        Self { sender }
    }

    pub async fn post(&self, message: Message) -> Result<(), MailboxProcessorError> {
        self.sender
            .send(message)
            .await
            .report()
            .change_context(MailboxProcessorError::SenderChannelClosed)
    }
}

async fn process_message(message: Message) {
    use futures::future::OptionFuture;

    match message {
        Message::CreateAccount { id, description, reply_channel } => {
            dbg!((id, description));
            OptionFuture::from(reply_channel.map(|rc| async { rc.send(()) })).await;
        },
    }
}

#[derive(Debug)]
pub enum Message {
    CreateAccount { id: u32, description: String, reply_channel: Option<sync::oneshot::Sender<()>> },
}
