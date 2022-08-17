pub use in_memory_store::InMemoryStore;

pub mod in_memory_store;

pub type EventProducer<T> = Box<dyn Fn(&[T]) -> Vec<T>>;

pub trait EventStorage<T> {
    fn append(&mut self, event: T);
    fn evolve(&mut self, producer: EventProducer<T>);
}

pub trait Query {
    type Item;

    fn all(&self) -> &[Self::Item];
}
