pub use in_memory_store::InMemoryStore;

pub mod in_memory_store;

pub trait EventStorage<T> {
    type Error;

    fn append(&mut self, event: T);
    fn evolve<F, C>(&mut self, producer: F) -> Result<(), Self::Error>
    where
        C: error_stack::Context,
        F: Fn(&[T]) -> error_stack::Result<Vec<T>, C>;
}

pub trait Query {
    type Item;

    fn all(&self) -> &[Self::Item];
}
