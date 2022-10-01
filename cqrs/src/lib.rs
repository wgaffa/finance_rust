use std::{collections::HashSet, ops::Neg};

use error::{AccountError, JournalError};
pub use events::Event;
pub use personal_finance::{balance::Balance, account::{Category, Name, Number}};

pub mod error;
pub mod events;
pub mod identifier;
pub mod stream;
pub mod write;

pub use write::chart::Chart;
pub use write::journal::Journal;

pub type JournalId = u32;
pub type AccountId = u32;
