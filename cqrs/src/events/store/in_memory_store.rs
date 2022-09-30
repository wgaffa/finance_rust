use super::{EventStorage, Query};

pub struct InMemoryStore<T> {
    data: Vec<T>,
}

impl<T> InMemoryStore<T> {
    pub fn new() -> InMemoryStore<T> {
        Self { data: Vec::new() }
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

    fn evolve<F>(&mut self, producer: F) -> Result<(), Self::Error>
    where
        F: Fn(&[T]) -> Vec<T>,
    {
        let new_events = producer(&self.data);
        self.data.extend(new_events);

        Ok(())
    }

    fn all(&self) -> &[T] {
        &self.data
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

impl<'a, T> IntoIterator for &'a InMemoryStore<T> {
    type IntoIter = std::slice::Iter<'a, T>;
    type Item = &'a T;

    fn into_iter(self) -> Self::IntoIter {
        self.data.iter()
    }
}

impl<T> Default for InMemoryStore<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> Extend<T> for InMemoryStore<T> {
    fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) {
        self.data.extend(iter);
    }
}
