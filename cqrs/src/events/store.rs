pub use in_memory_store::InMemoryStore;

pub mod in_memory_store;

pub trait EventStorage<T> {
    fn append(&mut self, event: T);
}

pub trait Query {
    type Item;

    fn all(&self) -> &[Self::Item];
}