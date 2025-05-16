use rdkafka::config::ClientConfig;
use rdkafka::producer::{BaseProducer, BaseRecord, Producer};
use ulid::Ulid;
use serde::Serialize;
use std::time::Duration;

use crate::traits::{event::EventEnvelope, Event, EventBus};

pub struct EventBusKafka {
    producer: BaseProducer,
}

impl EventBusKafka {
    pub fn new(bootstrap_servers: &str) -> Self {
        let producer: BaseProducer = ClientConfig::new()
            .set("bootstrap.servers", bootstrap_servers)
            .set("message.timeout.ms", "5000")
            .create()
            .expect("Failed to create producer");
        
        Self { producer }
    }
}

impl EventBus for EventBusKafka {
    fn produce_event<T, E: Event<T> + Serialize>(&self, event: E) -> Result<(), String> {
        let envelope = EventEnvelope::new(Ulid::new(), event.aggregate_id(), event.aggregate_type().to_string(), event);
        let envelope_json = serde_json::to_string(&envelope).map_err(|e| format!("Failed to serialize event: {}", e))?;

        self.producer
            .send(
                BaseRecord::to("events")
                    .payload(&envelope_json)
                    .key(&envelope.aggregate_id.to_string()),
            )
            .map_err(|(err, _)| format!("Failed to send event: {}", err))?;

        self.producer.flush(Duration::from_secs(5))
            .map_err(|e| format!("Failed to flush producer: {}", e))?;

        Ok(())
    }

    fn subscribe<T, E: Event<T>>(&self, event_type: &str, handler: Box<dyn Fn(E) -> Result<(), String> + Send + Sync>) {
        todo!()
    }
}
