use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;
use rusqlite::named_params;
use serde::{Deserialize, Serialize};
use ulid::Ulid;

use crate::traits::{Event, EventStore, event::EventEnvelope, event_store::EventStoreError};

#[derive(Debug, Clone)]
pub struct EventStoreSqlite {
    pool: Pool<SqliteConnectionManager>,
}

impl EventStoreSqlite {
    pub fn new(db_path: &str) -> Self {
        let manager = SqliteConnectionManager::file(db_path);
        let pool = Pool::new(manager).expect("Failed to create pool");

        // Apply migrations using pool.get()
        let conn = pool.get().expect("Failed to get connection");
        conn.execute_batch(
            "CREATE TABLE IF NOT EXISTS events (
                sequence_number TEXT PRIMARY KEY NOT NULL,
                aggregate_id TEXT NOT NULL,
                aggregate_type TEXT NOT NULL,
                event TEXT NOT NULL,
                created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
            );

            CREATE INDEX IF NOT EXISTS idx_events_aggregate
            ON events(aggregate_id, aggregate_type);",
        )
        .expect("Failed to create events table");

        Self { pool }
    }
}

impl EventStore for EventStoreSqlite {
    fn append_event<T, E: Event<T> + Serialize>(
        &self,
        aggregate_id: Ulid,
        aggregate_type: &str,
        event: E,
    ) -> Result<(), EventStoreError> {
        let conn = self.pool.get().expect("Failed to get connection");
        let sequence_number = Ulid::new();

        let envelope = EventEnvelope::new(
            sequence_number,
            aggregate_id,
            aggregate_type.to_string(),
            event.event_type().to_string(),
            event,
        );

        // Store the sequence number, aggregate_id, and aggregate_type as separate columns
        // but serialize just the event itself (not the whole envelope)
        let event_json = serde_json::to_string(&envelope)
            .map_err(|e| EventStoreError::EventStoreError(e.to_string()))?;

        let mut statement = conn
            .prepare(
                "INSERT INTO events (sequence_number, aggregate_id, aggregate_type, event) VALUES (:sequence_number, :aggregate_id, :aggregate_type, :event)",
            )
            .map_err(|e| EventStoreError::EventStoreError(e.to_string()))?;

        statement
            .execute(named_params! {
                ":sequence_number": sequence_number.to_string(),
                ":aggregate_id": aggregate_id.to_string(),
                ":aggregate_type": aggregate_type,
                ":event": event_json,
            })
            .map_err(|e| EventStoreError::EventStoreError(e.to_string()))?;

        Ok(())
    }

    fn get_events_for_aggregate<T, E: Event<T> + Serialize + for<'de> Deserialize<'de>>(
        &self,
        aggregate_id: Ulid,
        aggregate_type: &str,
    ) -> Result<Vec<EventEnvelope<T, E>>, EventStoreError> {
        let conn = self.pool.get().expect("Failed to get connection");
        let mut statement = conn
            .prepare(
                "SELECT sequence_number, aggregate_id, aggregate_type, event
             FROM events
             WHERE aggregate_id = :aggregate_id AND aggregate_type = :aggregate_type
             ORDER BY sequence_number",
            )
            .map_err(|e| EventStoreError::EventStoreError(e.to_string()))?;

        let rows = statement
            .query_map(
                named_params! {
                    ":aggregate_id": aggregate_id.to_string(),
                    ":aggregate_type": aggregate_type,
                },
                |row| {
                    let event_json: String = row.get(3)?;

                    // Deserialize just the event from the JSON string
                    let envelope: EventEnvelope<T, E> =
                        serde_json::from_str(&event_json).map_err(|e| {
                            rusqlite::Error::FromSqlConversionFailure(
                                3,
                                rusqlite::types::Type::Text,
                                Box::new(e),
                            )
                        })?;

                    // Create a new envelope with the event and metadata
                    Ok(envelope)
                },
            )
            .map_err(|e| EventStoreError::EventStoreError(e.to_string()))?;

        let mut events = Vec::new();
        for row in rows {
            events.push(row.map_err(|e| EventStoreError::EventStoreError(e.to_string()))?);
        }

        Ok(events)
    }

    fn get_all_events<T, E: Event<T> + Serialize + for<'de> Deserialize<'de>>(
        &self,
    ) -> Result<Vec<EventEnvelope<T, E>>, EventStoreError> {
        let conn = self.pool.get().expect("Failed to get connection");
        let mut statement = conn
            .prepare(
                "SELECT sequence_number, aggregate_id, aggregate_type, event FROM events ORDER BY sequence_number",
            )
            .map_err(|e| EventStoreError::EventStoreError(e.to_string()))?;

        let rows = statement
            .query_map([], |row| {
                let event_json: String = row.get(3)?;

                // Deserialize just the event from the JSON string
                let envelope: EventEnvelope<T, E> =
                    serde_json::from_str(&event_json).map_err(|e| {
                        rusqlite::Error::FromSqlConversionFailure(
                            3,
                            rusqlite::types::Type::Text,
                            Box::new(e),
                        )
                    })?;

                // Create a new envelope with the event and metadata
                Ok(envelope)
            })
            .map_err(|e| EventStoreError::EventStoreError(e.to_string()))?;

        let mut events = Vec::new();

        for row in rows {
            events.push(row.map_err(|e| EventStoreError::EventStoreError(e.to_string()))?);
        }

        Ok(events)
    }
}
