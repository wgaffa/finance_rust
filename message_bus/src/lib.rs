use async_trait::async_trait;
use cqrs::{Event, events::store::InMemoryStore};
use error_stack::{IntoReport, Result, ResultExt};
use tokio::{
    sync::{
        self,
        mpsc::{self, Sender},
    },
    task,
};

mod message;
mod command_handler;

pub use message::Message;
pub use command_handler::CommandHandler;

#[derive(Debug)]
pub enum MailboxProcessorError {
    MailboxProcessTerminated,
}

impl std::fmt::Display for MailboxProcessorError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Self::MailboxProcessTerminated => {
                f.write_str("Could not send message to mailbox process")
            }
        }
    }
}

impl std::error::Error for MailboxProcessorError {}

#[async_trait]
pub trait MessageProcessor<T> {
    async fn process_message(&mut self, message: T);
}

pub struct MailboxProcessor {
    sender: Sender<Message>,
}

impl MailboxProcessor
{
    pub async fn new<P>(mut message_processor: P) -> Self
    where
        P: MessageProcessor<Message> + Send + 'static
    {
        let (sender, mut receiver) = mpsc::channel(32);

        task::spawn(async move {
            loop {
                match receiver.recv().await {
                    None => break,
                    Some(message) => message_processor.process_message(message).await,
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
            .change_context(MailboxProcessorError::MailboxProcessTerminated)
    }
}

