pub use in_memory_store::InMemoryStore;

pub mod in_memory_store;

pub trait EventStorage<T> {
    type Error;

    fn append(&mut self, event: T);
    fn evolve<F>(&mut self, producer: F) -> Result<(), Self::Error>
    where
        F: Fn(&[T]) -> Vec<T>;

    fn all(&self) -> &[T];
}

pub trait Query {
    type Item;

    fn all(&self) -> &[Self::Item];
}
