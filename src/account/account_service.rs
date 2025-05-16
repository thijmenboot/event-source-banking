use rust_decimal::Decimal;
use ulid::Ulid;

use crate::account::Account;
use crate::traits::{Aggregate, Command, Event, EventBus, EventStore, Repository};

use crate::account::events::AccountEvent;

use super::commands::{DepositCommand, OpenAccountCommand, WithdrawCommand};

pub struct AccountService<R: Repository<Account>, E: EventStore, B: EventBus> {
    repository: R, // reading
    event_store: E, // writing
    event_bus: B, // publishing
}

impl<R: Repository<Account>, E: EventStore, B: EventBus> AccountService<R, E, B> {
    pub fn new(repository: R, event_store: E, event_bus: B) -> Self {
        Self { repository, event_store, event_bus }
    }

    pub fn create_account(&self, balance: Decimal) -> Result<Account, String> {
        let mut account = Account::default();

        let command = OpenAccountCommand {
            balance,
        };

        let events = command.execute(account.clone())?;

        for event in events {
            account = event.apply(account)?;

            // append event to the event store to save history and state
            self.event_store.append_event(account.account_id.ok_or("Account ID is required")?, "Account".to_string(), event.clone())?;

            // publish event for other services and projections to consume
            self.event_bus.produce_event(event)?;
        }

        Ok(account)
    }

    pub fn deposit(&self, account_id: Ulid, amount: Decimal) -> Result<(), String> {
        let events = self.event_store.get_events_for_aggregate(account_id, "Account".to_string())?;
        let mut account = Account::from_history::<AccountEvent>(events.into_iter().map(|e| e.event).collect())?;

        let command = DepositCommand {
            amount,
        };

        let events = command.execute(account.clone())?;

        for event in events {
            account = event.apply(account)?;

            // append event to the event store to save history and state
            self.event_store.append_event(account.account_id.ok_or("Account ID is required")?, "Account".to_string(), event.clone())?;

            // publish event for other services and projections to consume
            self.event_bus.produce_event(event)?;
        }

        Ok(())
    }

    pub fn withdraw(&self, account_id: Ulid, amount: Decimal) -> Result<(), String> {
        let events = self.event_store.get_events_for_aggregate(account_id, "Account".to_string())?;
        let mut account = Account::from_history::<AccountEvent>(events.into_iter().map(|e| e.event).collect())?;

        let command = WithdrawCommand {
            amount,
        };

        let events = command.execute(account.clone())?;

        for event in events {
            account = event.apply(account)?;

            // append event to the event store to save history and state
            self.event_store.append_event(account.account_id.ok_or("Account ID is required")?, "Account".to_string(), event.clone())?;

            // publish event for other services and projections to consume
            self.event_bus.produce_event(event)?;
        }

        Ok(())
    }

    pub fn get_account(&self, account_id: Ulid) -> Result<Account, String> {
        self.repository.get(account_id)
    }
}
