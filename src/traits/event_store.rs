use crate::Event;
use serde::{Deserialize, Serialize};
use ulid::Ulid;

use super::event::EventEnvelope;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum EventStoreError {
    #[error("Event store error: {0}")]
    EventStoreError(String),
}

pub trait EventStore {
    fn append_event<T, E: Event<T> + Serialize>(
        &self,
        aggregate_id: Ulid,
        aggregate_type: &str,
        event: E,
    ) -> Result<(), EventStoreError>;
    fn get_events_for_aggregate<T, E: Event<T> + Serialize + for<'de> Deserialize<'de>>(
        &self,
        aggregate_id: Ulid,
        aggregate_type: &str,
    ) -> Result<Vec<EventEnvelope<T, E>>, EventStoreError>;
    fn get_all_events<T, E: Event<T> + Serialize + for<'de> Deserialize<'de>>(
        &self,
    ) -> Result<Vec<EventEnvelope<T, E>>, EventStoreError>;
}
