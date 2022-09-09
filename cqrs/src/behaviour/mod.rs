use crate::events::Event;
use personal_finance::account::Category;

pub fn open_account(id: u32, name: String, category: Category, _events: &[Event]) -> Vec<Event> {
    vec![Event::AccountOpened {
        id,
        name,
        category: category,
    }]
}
