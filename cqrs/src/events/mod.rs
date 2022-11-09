use std::{sync::Arc, ops::Deref};

use super::JournalId;
use crate::write::ledger::LedgerId;
use chrono::prelude::*;
use personal_finance::{
    account::{Category, Name, Number},
    balance::Balance,
};

pub mod projections;
pub mod store;

pub type EventPointerType = <Event as EventPointer>::Pointer<Event>;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Event {
    LedgerCreated {
        id: LedgerId,
    },
    AccountOpened {
        ledger: LedgerId,
        id: Number,
        name: Name,
        category: Category,
    },
    AccountClosed {
        ledger: LedgerId,
        account: Number,
    },
    Transaction {
        ledger: LedgerId,
        description: String,
        date: Date<Utc>,
        transactions: Vec<(Number, Balance)>,
    },
}

pub trait EventPointer {
    type Pointer<T>: Deref<Target = T>;

    fn new<T>(value: T) -> Self::Pointer<T>;
}

impl EventPointer for Event {
    type Pointer<T> = Arc<T>;

    fn new<T>(value: T) -> Self::Pointer<T> {
        Arc::new(value)
    }
}
