use futures::future::OptionFuture;
use tokio::{
    sync::{
        self,
        mpsc::{self, Sender},
        oneshot,
    },
    task,
};

use crate::Message;
use cqrs::{
    error::{AccountError, JournalError},
    events::store::EventStorage,
    AccountId,
    Event,
    JournalId, Balance,
};
use personal_finance::{
    account::{Category, Name},
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

type Responder<U, E> = Option<oneshot::Sender<Result<U, E>>>;

impl<'a, T> CommandHandler<T>
where
    T: EventStorage<Event> + Extend<Event>,
{
    pub async fn send_reply<U, E>(&mut self, reply_channel: Responder<U, E>, reply: Result<U, E>) {
        OptionFuture::from(reply_channel.map(|rc| async { rc.send(reply) })).await;
    }

    pub async fn process_message(&mut self, message: Message) {
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
                reply_channel,
            } => {
                self.process_journal_entry_message(description, transactions, reply_channel)
                    .await
            }
        }
    }

    pub async fn process_create_account_message(
        &mut self,
        id: JournalId,
        description: String,
        category: Category,
        reply_channel: Responder<(), AccountError>,
    ) {
        let mut events = self.store_handle.all();
        let mut chart = cqrs::Chart::new(&events);
        let entry = chart.open(id.into(), Name::new(description).unwrap(), category);

        let entry = entry.map(|events| self.store_handle.extend(events.iter().cloned()));
        self.send_reply(reply_channel, entry).await;
    }

    pub async fn process_journal_entry_message(
        &mut self,
        description: String,
        transactions: Vec<(AccountId, Balance)>,
        reply_channel: Responder<JournalId, JournalError>,
    ) {
        let events = self.store_handle.all();
        let mut journal = cqrs::Journal::new(&events);
        let entry = journal.entry(description, &transactions);

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
