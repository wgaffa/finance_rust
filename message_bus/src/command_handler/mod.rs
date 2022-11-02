use std::{ops::Deref, sync::Arc};

use async_trait::async_trait;
use chrono::prelude::*;
use futures::future::OptionFuture;

use crate::{message::Responder, Message, MessageProcessor};
use cqrs::{
    error::{AccountError, LedgerError, TransactionError},
    events::store::EventStorage,
    write::ledger::LedgerId,
    Balance,
    Event,
    JournalId,
};
use personal_finance::account::{Category, Name, Number};

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
        ledger: LedgerId,
        id: Number,
        description: Name,
        category: Category,
        reply_channel: Responder<(), AccountError>,
    ) {
        let events = self
            .store_handle
            .all()
            .iter()
            .cloned()
            .map(Arc::new)
            .collect::<Vec<_>>();
        let entry = cqrs::Ledger::new(ledger, events.as_slice())
            .ok_or(AccountError::LedgerDoesnExist)
            .and_then(|mut ledger| {
                ledger
                    .open_account(id, description, category)
                    .map(|events| {
                        self.store_handle
                            .extend(events.iter().map(|x| x.deref().clone()))
                    })
            });

        self.send_reply(reply_channel, entry).await;
    }

    async fn process_transaction_message(
        &mut self,
        ledger: LedgerId,
        description: String,
        transactions: Vec<(Number, Balance)>,
        date: Date<Utc>,
        reply_channel: Responder<(), TransactionError>,
    ) {
        let events = self
            .store_handle
            .all()
            .iter()
            .cloned()
            .map(Arc::new)
            .collect::<Vec<_>>();
        let entry = cqrs::Ledger::new(ledger, &events)
            .ok_or(TransactionError::LedgerDoesnExist)
            .and_then(|mut ledger| {
                ledger
                    .transaction(description, &transactions, date)
                    .map(|events| {
                        self.store_handle
                            .extend(events.iter().map(Deref::deref).cloned())
                    })
            });

        self.send_reply(reply_channel, entry).await;
    }

    async fn process_close_account(
        &mut self,
        ledger: LedgerId,
        id: Number,
        reply_channel: Responder<(), AccountError>,
    ) {
        let events = self.store_handle.all();
        let events = events
            .iter()
            .map(|x| Arc::new(x.clone()))
            .collect::<Vec<_>>();
        let reply = cqrs::Ledger::new(ledger, events.as_slice())
            .ok_or(AccountError::LedgerDoesnExist)
            .and_then(|mut ledger| {
                ledger.close_account(id).map(|events| {
                    self.store_handle
                        .extend(events.iter().map(Deref::deref).cloned())
                })
            });

        self.send_reply(reply_channel, reply).await;
    }

    async fn process_create_ledger(
        &mut self,
        id: LedgerId,
        reply_channel: Responder<(), LedgerError>,
    ) {
        let events = self.store_handle.all();
        let mut resolver = cqrs::write::ledger::LedgerResolver::new(&events);

        let reply = resolver.create(id).map(|events| {
            self.store_handle.extend(events.iter().cloned());
        });

        self.send_reply(reply_channel, reply).await;
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
                ledger,
                id,
                description,
                category,
                reply_channel,
            } => {
                self.process_create_account_message(
                    ledger,
                    id,
                    description,
                    category,
                    reply_channel,
                )
                .await
            }
            Message::Transaction {
                ledger,
                description,
                transactions,
                date,
                reply_channel,
            } => {
                self.process_transaction_message(
                    ledger,
                    description,
                    transactions,
                    date,
                    reply_channel,
                )
                .await
            }
            Message::CloseAccount {
                ledger,
                id,
                reply_channel,
            } => self.process_close_account(ledger, id, reply_channel).await,
            Message::CreateLedger { id, reply_channel } => {
                self.process_create_ledger(id, reply_channel).await
            }
        }
    }
}
