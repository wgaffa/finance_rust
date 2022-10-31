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
        ledger: LedgerId,
        id: Number,
        description: Name,
        category: Category,
        reply_channel: Responder<(), cqrs::error::AccountError>,
    },
    Transaction {
        ledger: LedgerId,
        description: String,
        transactions: Vec<(Number, Balance)>,
        date: Date<Utc>,
        reply_channel: Responder<(), cqrs::error::TransactionError>,
    },
    CloseAccount {
        ledger: LedgerId,
        id: Number,
        reply_channel: Responder<(), cqrs::error::AccountError>,
    },
    CreateLedger {
        id: LedgerId,
        reply_channel: Responder<(), cqrs::error::LedgerError>,
    },
}
