use personal_finance::account::Category;
use crate::events::Event;

pub fn open_account(id: u32, name: String, category: Category, events: &[Event]) -> Vec<Event> {
    vec![Event::AccountOpened { id, name, category }]
}

