pub mod traits;
pub mod account;
pub mod event_store_sqlite;
pub mod event_bus_kafka;

use event_bus_kafka::EventBusKafka;
use event_store_sqlite::EventStoreSqlite;
use rust_decimal::Decimal;
use ulid::Ulid;

use account::{commands::{DepositCommand, OpenAccountCommand, WithdrawCommand}, events::{AccountEvent, AccountOpenedEvent, DepositEvent, WithdrawEvent}, repositories::AccountRepositorySqlite, Account, AccountService };
use traits::{Aggregate, Command, Event, EventStore, Repository};

struct Config {
    event_store_path: String,
    projection_database_path: String,
    kafka_bootstrap_servers: String,
}

impl Config {
    pub fn new(event_store_path: String, projection_database_path: String, kafka_bootstrap_servers: String) -> Self {
        Self { event_store_path, projection_database_path, kafka_bootstrap_servers }
    }
}

struct Application {
    account_service: AccountService<AccountRepositorySqlite, EventStoreSqlite, EventBusKafka>,
}

impl Application {
    pub fn new(config: Config) -> Self {
        Self { account_service: AccountService::new(AccountRepositorySqlite::new(&config.projection_database_path), EventStoreSqlite::new(&config.event_store_path), EventBusKafka::new(&config.kafka_bootstrap_servers)) }
    }
}

fn main() {
    // TODO: write event store implementation [x]
    // TODO: write event bus implementation [subscriber todo]
    // TODO: write event handlers to store aggregates in projection database
    // TODO: write migrations for databases [x]
    // TODO: write interfaces to trigger writing and reading of aggregates
    // TODO: make simple front-end (tech stack to be determined)
    let app = Application::new(Config::new(
        "data.db".to_string(),
        "data.db".to_string(),
        "localhost:9092".to_string(),
    ));

    // create account
    let account = app.account_service.create_account(Decimal::from(100)).expect("Failed to create account");
    let account_id = account.account_id.ok_or("Failed to get account id".to_string()).unwrap();

    // deposit 100 into the account
    app.account_service.deposit(account_id, Decimal::from(100)).expect("Failed to deposit");

}
