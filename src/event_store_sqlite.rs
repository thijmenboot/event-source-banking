use rusqlite::{named_params, Connection};
use ulid::Ulid;
use serde::{Serialize, Deserialize};

use crate::traits::{event::EventEnvelope, Event, EventStore};

pub struct EventStoreSqlite {
    db: Connection,
}

impl EventStoreSqlite {
    pub fn new(db_path: &str) -> Self {
        let db = Connection::open(db_path).expect("Failed to open database");

        // Apply migrations
        db.execute_batch(
            "CREATE TABLE IF NOT EXISTS events (
                sequence_number TEXT PRIMARY KEY NOT NULL,
                aggregate_id TEXT NOT NULL,
                aggregate_type TEXT NOT NULL,
                event TEXT NOT NULL,
                created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
            );
            
            CREATE INDEX IF NOT EXISTS idx_events_aggregate 
            ON events(aggregate_id, aggregate_type);"
        ).expect("Failed to create events table");

        Self { db }
    }
}

impl EventStore for EventStoreSqlite {
    fn append_event<T, E: Event<T> + Serialize>(&self, aggregate_id: Ulid, aggregate_type: String, event: E) -> Result<(), String> {
        let envelope = EventEnvelope::new(Ulid::new(), aggregate_id, aggregate_type, event);
        let event_json = serde_json::to_string(&envelope.event).map_err(|e| e.to_string())?;

        let mut statement = self.db.prepare("INSERT INTO events (sequence_number, aggregate_id, aggregate_type, event) VALUES (:sequence_number, :aggregate_id, :aggregate_type, :event)")
            .map_err(|e| format!("Failed to prepare statement: {}", e))?;
        
        statement.execute(named_params! {
            ":sequence_number": envelope.sequence_number.to_string(),
            ":aggregate_id": envelope.aggregate_id.to_string(),
            ":aggregate_type": envelope.aggregate_type,
            ":event": event_json,
        }).map_err(|e| format!("Failed to execute statement: {}", e))?;
        
        Ok(())
    }
    
    fn get_events_for_aggregate<T, E: Event<T> + Serialize + for<'de> Deserialize<'de>>(&self, aggregate_id: Ulid, aggregate_type: String) -> Result<Vec<EventEnvelope<T, E>>, String> {
        let mut statement = self.db.prepare(
            "SELECT sequence_number, aggregate_id, aggregate_type, event 
             FROM events 
             WHERE aggregate_id = :aggregate_id AND aggregate_type = :aggregate_type
             ORDER BY sequence_number"
        ).map_err(|e| format!("Failed to prepare statement: {}", e))?;
        
        let rows = statement.query_map(named_params! {
            ":aggregate_id": aggregate_id.to_string(),
            ":aggregate_type": aggregate_type,
        }, |row| {
            let sequence_number: String = row.get("sequence_number")?;
            let aggregate_id: String = row.get("aggregate_id")?;
            let aggregate_type: String = row.get("aggregate_type")?;
            let event_json: String = row.get("event")?;
            
            Ok(EventEnvelope::new(
                Ulid::from_string(&sequence_number).map_err(|e| rusqlite::Error::FromSqlConversionFailure(0, rusqlite::types::Type::Text, Box::new(e)))?,
                Ulid::from_string(&aggregate_id).map_err(|e| rusqlite::Error::FromSqlConversionFailure(1, rusqlite::types::Type::Text, Box::new(e)))?,
                aggregate_type,
                serde_json::from_str(&event_json).map_err(|e| rusqlite::Error::FromSqlConversionFailure(3, rusqlite::types::Type::Text, Box::new(e)))?
            ))
        }).map_err(|e| format!("Failed to query events: {}", e))?;

        let mut events = Vec::new();    
        for row in rows {
            events.push(row.map_err(|e| format!("Failed to process row: {}", e))?);
        }

        Ok(events)
    }
    
    fn get_all_events<T, E: Event<T> + Serialize + for<'de> Deserialize<'de>>(&self) -> Result<Vec<EventEnvelope<T, E>>, String> {
        let mut statement = self.db.prepare("SELECT sequence_number, aggregate_id, aggregate_type, event FROM events ORDER BY sequence_number")
            .map_err(|e| format!("Failed to prepare statement: {}", e))?;
        
        let rows = statement.query_map([], |row| {
            let sequence_number: String = row.get("sequence_number")?;
            let aggregate_id: String = row.get("aggregate_id")?;
            let aggregate_type: String = row.get("aggregate_type")?;
            let event_json: String = row.get("event")?;

            Ok(EventEnvelope::new(
                Ulid::from_string(&sequence_number).map_err(|e| rusqlite::Error::FromSqlConversionFailure(0, rusqlite::types::Type::Text, Box::new(e)))?,
                Ulid::from_string(&aggregate_id).map_err(|e| rusqlite::Error::FromSqlConversionFailure(1, rusqlite::types::Type::Text, Box::new(e)))?,
                aggregate_type,
                serde_json::from_str(&event_json).map_err(|e| rusqlite::Error::FromSqlConversionFailure(3, rusqlite::types::Type::Text, Box::new(e)))?
            ))
        }).map_err(|e| format!("Failed to query events: {}", e))?;

        let mut events = Vec::new();
        
        for row in rows {
            events.push(row.map_err(|e| format!("Failed to process row: {}", e))?);
        }

        Ok(events)
    }
}