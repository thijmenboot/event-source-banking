use rdkafka::config::ClientConfig;
use rdkafka::producer::{BaseProducer, BaseRecord, Producer};
use rdkafka::consumer::{BaseConsumer, Consumer};
use rdkafka::Message;
use ulid::Ulid;
use serde::{Deserialize, Serialize};
use std::time::Duration;
use std::sync::Arc;
use crate::traits::{event::EventEnvelope, Event, EventBus};

#[derive(Clone)]
pub struct EventBusKafka {
    producer: Arc<BaseProducer>,
    consumer: Arc<BaseConsumer>,
}

impl EventBusKafka {
    pub fn new(bootstrap_servers: &str) -> Self {
        // Use a stable consumer group ID instead of a random one
        let consumer_group = "banking_consumer_group";

        let producer: BaseProducer = ClientConfig::new()
            .set("bootstrap.servers", bootstrap_servers)
            .set("message.timeout.ms", "5000")
            .create()
            .expect("Failed to create producer");

        let consumer: BaseConsumer = ClientConfig::new()
            .set("bootstrap.servers", bootstrap_servers)  // Should match producer config
            .set("group.id", consumer_group)
            .set("enable.auto.commit", "true")
            .set("auto.offset.reset", "earliest") // Process existing messages only once per consumer group
            .create()
            .expect("Failed to create consumer");

        consumer.subscribe(&["events"])
            .expect("Failed to subscribe to topic");
        
        Self { producer: Arc::new(producer), consumer: Arc::new(consumer) }
    }
}

impl EventBus for EventBusKafka {
    fn produce_event<T, E: Event<T> + Serialize>(&self, event: E) -> Result<(), String> {
        let envelope = EventEnvelope::new(Ulid::new(), event.aggregate_id(), event.aggregate_type().to_string(), event.event_type().to_string(), event);
        let envelope_json = serde_json::to_string(&envelope).map_err(|e| format!("Failed to serialize event: {}", e))?;

        self.producer
            .send(
                BaseRecord::to("events")
                    .payload(&envelope_json)
                    .key(&envelope.aggregate_id.to_string()),
            )
            .map_err(|(err, _)| format!("Failed to send event: {}", err))?;

        Ok(())
    }

    fn subscribe<T, E>(&self, aggregate_type: &str, handler: Box<dyn Fn(E) -> Result<(), String> + Send + Sync + 'static>) 
    where 
        E: Event<T> + for<'de> Deserialize<'de> + 'static 
    {
        let aggregate_type = aggregate_type.to_string();
        let consumer = self.consumer.clone();  // Clone the Arc<BaseConsumer>
                
        std::thread::spawn(move || {
            loop {
                match consumer.poll(Duration::from_millis(50)) {
                    Some(Ok(msg)) => {
                        if let Some(payload) = msg.payload() {
                            {
                                let payload_vec = payload.to_vec();
                                match serde_json::from_slice::<EventEnvelope<T, E>>(&payload_vec) {
                                    Ok(envelope) => {
                                        if envelope.event.aggregate_type() == aggregate_type {
                                            if let Err(e) = handler(envelope.event) {
                                                eprintln!("Error handling event: {}", e);
                                            }
                                        }
                                    },
                                    Err(e) => eprintln!("Failed to deserialize: {}", e),
                                }
                            }
                        }
                    }
                    Some(Err(e)) => eprintln!("Error while receiving message: {}", e),
                    None => {}
                }
            }
        });
    }
}
