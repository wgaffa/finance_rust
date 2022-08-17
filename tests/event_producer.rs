use cqrs::events::{store::{EventStorage, InMemoryStore, EventProducer}, Event};

#[test]
fn create_new_account_in_empty_chart() {
    let add_event: EventProducer<Event> = Box::new(|_events| vec![Event::AccountClosed(101)]);
    let mut repo = InMemoryStore::new();

    repo.evolve(add_event);
    let current_events = repo.iter().cloned().collect::<Vec<_>>();

    assert_eq!(current_events, [Event::AccountClosed(101)].to_vec());
}
