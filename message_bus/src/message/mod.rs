use tokio::sync;

use cqrs::events::Balance;
use personal_finance::account::Category;

type Responder<T, E> = Option<sync::oneshot::Sender<Result<T, E>>>;

#[derive(Debug)]
pub enum Message {
    CreateAccount {
        id: u32,
        description: String,
        category: Category,
        reply_channel: Responder<(), cqrs::error::AccountError>,
    },
    JournalEntry {
        description: String,
        transactions: Vec<(u32, Balance)>,
        reply_channel: Responder<usize, cqrs::error::JournalError>,
    },
}
