use rust_decimal::Decimal;
use ulid::Ulid;

use crate::account::Account;
use crate::traits::{Aggregate, Command, EventStore, Repository};

use crate::account::events::AccountEvent;

use super::commands::{DepositCommand, OpenAccountCommand, WithdrawCommand};

pub struct AccountService<R: Repository<Account>, E: EventStore> {
    repository: R, // reading
    event_store: E, // writing
}

impl<R: Repository<Account>, E: EventStore> AccountService<R, E> {
    pub fn new(repository: R, event_store: E) -> Self {
        Self { repository, event_store }
    }

    pub fn create_account(&self, balance: Decimal) -> Result<(), String> {
        let account = Account::from_history::<AccountEvent>(vec![])?;

        let command = OpenAccountCommand {
            balance,
        };

        let events = command.execute(account.clone())?;

        for event in events {
            self.event_store.append_event(account.account_id.ok_or("Account ID is required")?, "Account".to_string(), event)?;
        }

        Ok(())
    }

    pub fn deposit(&self, account_id: Ulid, amount: Decimal) -> Result<(), String> {
        let events = self.event_store.get_events_for_aggregate(account_id, "Account".to_string())?;
        let account = Account::from_history::<AccountEvent>(events.into_iter().map(|e| e.event).collect())?;

        let command = DepositCommand {
            amount,
        };

        let events = command.execute(account.clone())?;

        for event in events {
            self.event_store.append_event(account.account_id.ok_or("Account ID is required")?, "Account".to_string(), event)?;
        }

        Ok(())
    }

    pub fn withdraw(&self, account_id: Ulid, amount: Decimal) -> Result<(), String> {
        let events = self.event_store.get_events_for_aggregate(account_id, "Account".to_string())?;
        let account = Account::from_history::<AccountEvent>(events.into_iter().map(|e| e.event).collect())?;

        let command = WithdrawCommand {
            amount,
        };

        let events = command.execute(account.clone())?;

        for event in events {
            self.event_store.append_event(account.account_id.ok_or("Account ID is required")?, "Account".to_string(), event)?;
        }

        Ok(())
    }

    pub fn get_account(&self, account_id: Ulid) -> Result<Account, String> {
        self.repository.get(account_id)
    }
}
