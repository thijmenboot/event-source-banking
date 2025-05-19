use super::Event;
use serde::{Deserialize, Serialize};
use thiserror::Error;
#[derive(Debug, Error)]
pub enum EventBusError {
    #[error("Error producing event: {0}")]
    ProduceError(String),
    #[error("Error subscribing to event: {0}")]
    SubscribeError(String),
    #[error("Serialisation error: {0}")]
    SerialisationError(serde_json::Error),
    #[error("Error handling event: {0}")]
    HandleError(String),
}

pub trait EventBus {
    fn produce_event<T, E: Event<T> + Serialize>(&self, event: E) -> Result<(), EventBusError>;
    fn subscribe<T, E>(
        &self,
        event_type: &str,
        handler: Box<dyn Fn(E) -> Result<(), EventBusError> + Send + Sync + 'static>,
    ) where
        E: Event<T> + for<'de> Deserialize<'de> + 'static;
}
