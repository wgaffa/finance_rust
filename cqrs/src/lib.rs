pub use events::Event;
pub use personal_finance::{
    account::{Category, Name, Number},
    balance::Balance,
};

pub mod error;
pub mod events;
pub mod identifier;
pub mod stream;
pub mod write;
pub mod projections;

pub use write::ledger::Ledger;

pub type JournalId = u32;
