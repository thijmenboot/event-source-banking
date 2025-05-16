use serde::{Deserialize, Serialize};
use ulid::Ulid;
use crate::Event;

use super::event::EventEnvelope;

pub trait EventStore {
    fn append_event<T, E: Event<T> + Serialize>(&self, aggregate_id: Ulid, aggregate_type: &str, event: E) -> Result<(), String>;
    fn get_events_for_aggregate<T, E: Event<T> + Serialize + for<'de> Deserialize<'de>>(&self, aggregate_id: Ulid, aggregate_type: &str) -> Result<Vec<EventEnvelope<T, E>>, String>;
    fn get_all_events<T, E: Event<T> + Serialize + for<'de> Deserialize<'de>>(&self) -> Result<Vec<EventEnvelope<T, E>>, String>;
}