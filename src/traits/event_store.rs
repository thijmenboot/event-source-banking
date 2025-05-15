use ulid::Ulid;
use std::marker::PhantomData;

use crate::Event;

pub struct EventEnvelope<T, E: Event<T>> {
    pub sequence_number: Ulid,
    pub aggregate_id: Ulid,
    pub aggregate_type: String,
    pub event: E,
    pub _phantom: PhantomData<T>,
}

pub trait EventStore {
    fn append_event<T, E: Event<T>>(&self, aggregate_id: Ulid, aggregate_type: String, event: E) -> Result<(), String>;
    fn get_events_for_aggregate<T, E: Event<T>>(&self, aggregate_id: Ulid, aggregate_type: String) -> Result<Vec<EventEnvelope<T, E>>, String>;
    fn get_all_events<T, E: Event<T>>(&self) -> Result<Vec<EventEnvelope<T, E>>, String>;
}