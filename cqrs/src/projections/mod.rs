use std::collections::HashSet;
use crate::{write::ledger::LedgerId, Event};

pub fn ledger_ids(mut state: HashSet<LedgerId>, item: &Event) -> HashSet<LedgerId> {
    match item {
        Event::LedgerCreated{ id } => { state.insert(id.clone()); },
        _ => {}
    }

    state
}
