use async_trait::async_trait;
use chrono::prelude::*;
use futures::future::OptionFuture;
use tokio::{
    sync::{
        self,
        mpsc::{self, Sender},
        oneshot,
    },
    task,
};

use crate::{message::Responder, Message, MessageProcessor};
use cqrs::{
    error::{AccountError, JournalError},
    events::store::EventStorage,
    AccountId,
    Balance,
    Event,
    JournalId,
};
use personal_finance::{
    account::{Category, Name, Number},
    entry::Journal,
};

pub struct CommandHandler<T> {
    store_handle: T,
}

impl<T> CommandHandler<T>
where
    T: EventStorage<Event>,
{
    pub fn new(store_handle: T) -> Self {
        Self { store_handle }
    }
}

impl<'a, T> CommandHandler<T>
where
    T: EventStorage<Event> + Extend<Event>,
{
    async fn send_reply<U, E>(&mut self, reply_channel: Responder<U, E>, reply: Result<U, E>) {
        OptionFuture::from(reply_channel.map(|rc| async { rc.send(reply) })).await;
    }

    async fn process_create_account_message(
        &mut self,
        id: Number,
        description: Name,
        category: Category,
        reply_channel: Responder<(), AccountError>,
    ) {
        let mut events = self.store_handle.all();
        let mut chart = cqrs::Chart::new(&events);
        let entry = chart.open(id, description, category);

        let entry = entry.map(|events| self.store_handle.extend(events.iter().cloned()));
        self.send_reply(reply_channel, entry).await;
    }

    async fn process_journal_entry_message(
        &mut self,
        description: String,
        transactions: Vec<(Number, Balance)>,
        date: Date<Utc>,
        reply_channel: Responder<JournalId, JournalError>,
    ) {
        let events = self.store_handle.all();
        let mut journal = cqrs::Journal::new(&events);
        let entry = journal.entry(description, &transactions, date);

        let entry = entry.and_then(|events| {
            if let Some(cqrs::Event::Journal { id, .. }) = events
                .iter()
                .find(|e| matches!(e, cqrs::Event::Journal { .. }))
            {
                self.store_handle.extend(events.iter().cloned());
                Ok(*id)
            } else {
                Err(JournalError::NoJournalEvent)
            }
        });
        self.send_reply(reply_channel, entry).await;
    }
}

#[async_trait]
impl<T> MessageProcessor<Message> for CommandHandler<T>
where
    T: EventStorage<Event> + Extend<Event> + Send,
{
    async fn process_message(&mut self, message: Message) {
        match message {
            Message::CreateAccount {
                id,
                description,
                category,
                reply_channel,
            } => {
                self.process_create_account_message(id, description, category, reply_channel)
                    .await
            }
            Message::JournalEntry {
                description,
                transactions,
                date,
                reply_channel,
            } => {
                self.process_journal_entry_message(description, transactions, date, reply_channel)
                    .await
            }
        }
    }
}
