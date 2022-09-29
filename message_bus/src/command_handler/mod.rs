use tokio::{
    sync::{
        self,
        mpsc::{self, Sender},
    },
    task,
};

use crate::Message;
use cqrs::{events::store::EventStorage, Event};
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
    T: EventStorage<Event>,
{
    pub async fn process_message(&'a mut self, message: Message) {
        use futures::future::OptionFuture;

        match message {
            Message::CreateAccount {
                id,
                description,
                category,
                reply_channel,
            } => {
                let events = self.store_handle.all().cloned().collect::<Vec<_>>();
                let mut chart = cqrs::Chart::new(&events);
                let entry = chart.open(id.into(), Name::new(description).unwrap(), category);

                match entry {
                    Ok(events) => {
                        for event in events {
                            self.store_handle.append(event.clone());
                        }

                        OptionFuture::from(reply_channel.map(|rc| async { rc.send(Ok(())) })).await;
                    }
                    Err(e) => {
                        OptionFuture::from(reply_channel.map(|rc| async { rc.send(Err(e)) })).await;
                    }
                }
            }
            Message::JournalEntry {
                description,
                transactions,
                reply_channel,
            } => {
                let events = self.store_handle.all().cloned().collect::<Vec<_>>();
                let mut journal = cqrs::Journal::new(&events);
                let entry = dbg!(journal.entry(description, &transactions));

                match entry {
                    Ok(events) => {
                        if let cqrs::Event::Journal { id, .. } = events
                            .iter()
                            .find(|e| matches!(e, cqrs::Event::Journal { .. }))
                            .unwrap()
                        {
                            for event in events {
                                self.store_handle.append(event.clone());
                            }

                            OptionFuture::from(
                                reply_channel.map(|rc| async { rc.send(Ok(*id as usize)) }),
                            )
                            .await;
                        }
                    }
                    Err(e) => {
                        OptionFuture::from(reply_channel.map(|rc| async { rc.send(Err(e)) })).await;
                    }
                };
            }
        }
    }
}
