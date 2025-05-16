pub mod traits;
pub mod account;

use rust_decimal::Decimal;
use ulid::Ulid;

use account::{commands::{DepositCommand, OpenAccountCommand, WithdrawCommand}, events::{AccountEvent, AccountOpenedEvent, DepositEvent, WithdrawEvent}, repositories::AccountRepositorySqlite, Account, AccountService };
use traits::{Aggregate, Command, Event, EventStore, Repository};

struct Application {
    account_service: AccountService<AccountRepositorySqlite, EventStoreSqlite>,
}

impl Application {
    pub fn new() -> Self {
        Self { account_service: AccountService::new(AccountRepositorySqlite::new("projection.db"), EventStoreSqlite::new()) }
    }
}

fn main() {
    // TODO: write event store implementation
    // TODO: write event handlers to store aggregates in projection database
    // TODO: write migrations for databases
    // TODO: write interfaces to trigger writing and reading of aggregates
    let app = Application::new();

    app.account_service.create_account(Decimal::from(100)).expect("Failed to create account");

    // DEMO 1 
    let account_id = Ulid::new();
    let history: Vec<AccountEvent> = vec![
        AccountEvent::Opened(AccountOpenedEvent {
            account_id,
            balance: Decimal::from(100),
        }),
        AccountEvent::Deposited(DepositEvent {
            account_id,
            amount: Decimal::from(50),
        }),
        AccountEvent::Withdrawn(WithdrawEvent {
            account_id,
            amount: Decimal::from(25),
        }),
    ];

    // balance at this time: 125
    let mut account = Account::from_history(history).expect("Failed to create account from event history");

    // Deposit 100
    let command = DepositCommand {
        amount: Decimal::from(100),
    };

    // Execute command
    let events = command.execute(account.clone()).expect("Failed to execute command");

    // Apply events
    for event in events {
        account = event.apply(account).expect("Failed to handle event");
    }

    // Balance at this time: 225
    println!("Account: {:?}", account);
}
