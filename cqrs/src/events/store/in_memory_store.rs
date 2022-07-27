use super::EventStorage;

pub struct InMemoryStore<T> {
    data: Vec<T>,
}

impl<T> InMemoryStore<T> {
    pub fn new() -> InMemoryStore<T> {
        Self {
            data: Vec::new(),
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.data.iter()
    }
}

impl<T> EventStorage<T> for InMemoryStore<T> {
    fn append(&mut self, event: T) {
        self.data.push(event)
    }
}

impl<T> IntoIterator for InMemoryStore<T> {
    type IntoIter = std::vec::IntoIter<T>;
    type Item = T;

    fn into_iter(self) -> Self::IntoIter {
        self.data.into_iter()
    }
}
