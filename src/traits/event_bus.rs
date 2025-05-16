use serde::Serialize;
use super::Event;

pub trait EventBus {
    fn produce_event<T, E: Event<T> + Serialize>(&self, event: E) -> Result<(), String>;
    fn subscribe<T, E: Event<T>>(&self, event_type: &str, handler: Box<dyn Fn(E) -> Result<(), String> + Send + Sync>);
}