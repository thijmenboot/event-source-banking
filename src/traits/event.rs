use std::marker::PhantomData;

use serde::{Deserialize, Serialize};
use thiserror::Error;
use ulid::Ulid;

// Definition of ApplyError using thiserror
#[derive(Debug, Error)]
pub enum ApplyError {
    #[error("Event application validation error: {0}")]
    Validation(String),
    #[error("Event application invariant violated: {0}")]
    InvariantViolated(String),
}
// End of ApplyError definition

pub trait Event<T> {
    fn aggregate_id(&self) -> Ulid;
    fn aggregate_type(&self) -> &str;
    fn event_type(&self) -> &str;
    fn apply(&self, state: &mut T) -> Result<(), ApplyError>;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventEnvelope<T, E: Event<T>> {
    pub sequence_number: Ulid,
    pub aggregate_id: Ulid,
    pub aggregate_type: String,
    pub event: E,
    pub event_type: String,
    #[serde(skip)]
    pub _phantom: PhantomData<T>,
}

impl<T, E: Event<T>> EventEnvelope<T, E> {
    pub fn new(
        sequence_number: Ulid,
        aggregate_id: Ulid,
        aggregate_type: String,
        event_type: String,
        event: E,
    ) -> Self {
        Self {
            sequence_number,
            aggregate_id,
            aggregate_type,
            event_type,
            event,
            _phantom: PhantomData,
        }
    }

    pub fn aggregate_id(&self) -> Ulid {
        self.aggregate_id
    }

    pub fn aggregate_type(&self) -> &str {
        &self.aggregate_type
    }

    pub fn event(&self) -> &E {
        &self.event
    }

    pub fn sequence_number(&self) -> Ulid {
        self.sequence_number
    }

    pub fn event_type(&self) -> &str {
        &self.event_type
    }
}

// impl<T: Default> Event<T> for T {
//     fn apply(&self, state: T) -> Result<T, String> {
//         Ok(state)
//     }
// }
