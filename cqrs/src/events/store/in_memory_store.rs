use error_stack::ResultExt;

use super::{EventStorage, Query};

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

#[derive(Debug)]
pub struct EvolveError;

impl std::fmt::Display for EvolveError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Could not evolve")
    }
}

impl std::error::Error for EvolveError {}

impl<T> EventStorage<T> for InMemoryStore<T> {
    type Error = error_stack::Report<EvolveError>;

    fn append(&mut self, event: T) {
        self.data.push(event)
    }

    fn evolve<F, C>(&mut self, producer: F) -> Result<(), Self::Error>
    where
        C: error_stack::Context,
        F: Fn(&[T]) -> error_stack::Result<Vec<T>, C>,
    {
        let new_events = producer(&self.data).change_context(EvolveError)?;
        self.data.extend(new_events);

        Ok(())
    }
}

impl<T> Query for InMemoryStore<T> {
    type Item = T;

    fn all(&self) -> &[T] {
        &self.data
    }
}

impl<T> IntoIterator for InMemoryStore<T> {
    type IntoIter = std::vec::IntoIter<T>;
    type Item = T;

    fn into_iter(self) -> Self::IntoIter {
        self.data.into_iter()
    }
}
