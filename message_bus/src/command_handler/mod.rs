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
use cqrs::{error::JournalError, events::store::EventStorage, Event};
use personal_finance::account::Name;

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
    pub async fn send_reply<U, E>(
        &mut self,
        reply_channel: Option<oneshot::Sender<Result<U, E>>>,
        reply: Result<U, E>,
    ) {
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
                let mut events = self.store_handle.all();
                let mut chart = cqrs::Chart::new(&events);
                let entry = chart.open(id.into(), Name::new(description).unwrap(), category);

                let entry = entry.map(|events| self.store_handle.extend(events.iter().cloned()));
                self.send_reply(reply_channel, entry).await;
            }
            Message::JournalEntry {
                description,
                transactions,
                reply_channel,
            } => {
                let events = self.store_handle.all();
                let mut journal = cqrs::Journal::new(&events);
                let entry = journal.entry(description, &transactions);

                let entry = entry.and_then(|events| {
                    if let Some(cqrs::Event::Journal { id, .. }) = events
                        .iter()
                        .find(|e| matches!(e, cqrs::Event::Journal { .. }))
                    {
                        self.store_handle.extend(events.iter().cloned());
                        Ok(*id as usize)
                    } else {
                        Err(JournalError::NoJournalEvent)
                    }
                });
                self.send_reply(reply_channel, entry).await;
            }
        }
    }
}
