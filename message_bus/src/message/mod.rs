use chrono::prelude::*;
use tokio::sync;

use cqrs::JournalId;
use personal_finance::{balance::Balance, account::{Category, Name, Number}};

pub type Responder<T, E> = Option<sync::oneshot::Sender<Result<T, E>>>;

#[derive(Debug)]
pub enum Message {
    CreateAccount {
        id: Number,
        description: Name,
        category: Category,
        reply_channel: Responder<(), cqrs::error::AccountError>,
    },
    JournalEntry {
        description: String,
        transactions: Vec<(Number, Balance)>,
        date: Date<Utc>,
        reply_channel: Responder<JournalId, cqrs::error::JournalError>,
    },
}
