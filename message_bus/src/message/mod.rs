use chrono::prelude::*;
use tokio::sync;

use cqrs::{write::ledger::LedgerId, JournalId};
use personal_finance::{
    account::{Category, Name, Number},
    balance::Balance,
};

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
    CloseAccount {
        id: Number,
        reply_channel: Responder<(), cqrs::error::AccountError>,
    },
    CreateLedger {
        id: LedgerId,
        reply_channel: Responder<(), cqrs::error::LedgerError>,
    },
}
