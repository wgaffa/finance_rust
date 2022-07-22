use super::*;

pub struct EventStore {
    data: Vec<Event>,
}

impl EventStore {
    pub fn new() -> EventStore {
        Self {
            data: Vec::new(),
        }
    }

    pub fn push(&mut self, event: Event) {
        self.data.push(event)
    }

    pub fn iter(&self) -> impl Iterator<Item = &Event> {
        self.data.iter()
    }
}

impl IntoIterator for EventStore {
    type IntoIter = std::vec::IntoIter<Event>;
    type Item = Event;

    fn into_iter(self) -> Self::IntoIter {
        self.data.into_iter()
    }
}
