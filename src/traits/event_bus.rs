use serde::{Deserialize, Serialize};
use super::Event;

pub trait EventBus {
    fn produce_event<T, E: Event<T> + Serialize>(&self, event: E) -> Result<(), String>;
    fn subscribe<T, E>(&self, event_type: &str, handler: Box<dyn Fn(E) -> Result<(), String> + Send + Sync + 'static>) 
    where 
        E: Event<T> + for<'de> Deserialize<'de> + 'static;
}