pub mod account;
pub mod event_bus_kafka;
pub mod event_store_sqlite;
pub mod traits;

use account::{Account, AccountHandler, AccountService, repositories::AccountRepositorySqlite};
use event_bus_kafka::EventBusKafka;
use event_store_sqlite::EventStoreSqlite;
use rust_decimal::Decimal;
use std::thread;
use traits::{Aggregate, Event};

struct Config {
    event_store_path: String,
    projection_database_path: String,
    kafka_bootstrap_servers: String,
}

impl Config {
    pub fn new(
        event_store_path: String,
        projection_database_path: String,
        kafka_bootstrap_servers: String,
    ) -> Self {
        Self {
            event_store_path,
            projection_database_path,
            kafka_bootstrap_servers,
        }
    }
}

fn main() {
    // TODO: BALANCE NOT BEING CALCULATED CORRECTLY: IS 100 SHOULD BE 200
    // TODO: on startup have repositories seed projections with data from event store []
    // TODO: write interfaces to trigger writing and reading of aggregates []
    // TODO: make simple front-end (tech stack to be determined) []
    let config = Config::new(
        "data.db".to_string(),
        "data.db".to_string(),
        "localhost:9092".to_string(),
    );

    // construct necessary components
    let event_store = EventStoreSqlite::new(&config.event_store_path);
    let event_bus = EventBusKafka::new(&config.kafka_bootstrap_servers);

    // account components
    let account_repository = AccountRepositorySqlite::new(&config.projection_database_path);
    let account_service = AccountService::new(
        account_repository.clone(),
        event_store.clone(),
        event_bus.clone(),
    );
    let account_handler = AccountHandler::new(
        account_repository.clone(),
        event_bus.clone(),
        event_store.clone(),
    );

    // start application
    thread::spawn(move || {
        account_handler.listen();
    });

    // create account
    let account = account_service
        .create_account(Decimal::from(100))
        .expect("Failed to create account");
    let account_id = account
        .account_id
        .ok_or("Failed to get account id".to_string())
        .unwrap();

    // deposit 100 into the account
    account_service
        .deposit(account_id, Decimal::from(100))
        .expect("Failed to deposit");

    // Keep the main thread alive to prevent the application from exiting
    println!("Application started. Listening for events...");
    loop {
        std::thread::sleep(std::time::Duration::from_secs(1));
    }
}
