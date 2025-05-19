use rust_decimal::Decimal;
use ulid::Ulid;

use crate::account::Account;
use crate::traits::event::ApplyError;
use crate::traits::event_bus::EventBusError;
use crate::traits::event_store::EventStoreError;
use crate::traits::repository::RepositoryError;
use crate::traits::{Aggregate, Command, Event, EventBus, EventStore, Repository};

use crate::account::events::AccountEvent;

use super::commands::{
    DepositCommand, DepositError, OpenAccountCommand, OpenAccountError, WithdrawCommand,
    WithdrawError,
};
use super::events::ACCOUNT_AGGREGATE_TYPE;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum AccountServiceError {
    #[error("Event application error: {0}")]
    ApplyError(#[from] ApplyError),
    #[error("Event store error: {0}")]
    EventStoreError(#[from] EventStoreError),
    #[error("Event bus error: {0}")]
    EventBusError(#[from] EventBusError),
    #[error("Repository error: {0}")]
    RepositoryError(#[from] RepositoryError),
    #[error("Open account command error: {0}")]
    OpenAccountError(#[from] OpenAccountError),
    #[error("Deposit command error: {0}")]
    DepositError(#[from] DepositError),
    #[error("Withdraw command error: {0}")]
    WithdrawError(#[from] WithdrawError),
    #[error("Operation error: {0}")]
    OperationError(String),
}

pub struct AccountService<R: Repository<Account>, E: EventStore, B: EventBus> {
    repository: R,  // reading
    event_store: E, // writing
    event_bus: B,   // publishing
}

impl<R: Repository<Account>, E: EventStore, B: EventBus> AccountService<R, E, B> {
    pub fn new(repository: R, event_store: E, event_bus: B) -> Self {
        Self {
            repository,
            event_store,
            event_bus,
        }
    }

    pub fn create_account(&self, balance: Decimal) -> Result<Account, AccountServiceError> {
        let mut account = Account::default();

        let command = OpenAccountCommand { balance };

        let events = command.execute(account.clone())?;

        for event in events {
            event.apply(&mut account)?;

            self.event_store.append_event(
                account.account_id.ok_or_else(|| {
                    AccountServiceError::OperationError(
                        "Account ID is required after creation".to_string(),
                    )
                })?,
                ACCOUNT_AGGREGATE_TYPE,
                event.clone(),
            )?;

            self.event_bus.produce_event(event)?;
        }

        Ok(account)
    }

    pub fn deposit(&self, account_id: Ulid, amount: Decimal) -> Result<(), AccountServiceError> {
        let events_envelopes = self
            .event_store
            .get_events_for_aggregate(account_id, ACCOUNT_AGGREGATE_TYPE)?;
        let mut account = Account::from_history::<AccountEvent>(
            events_envelopes.into_iter().map(|e| e.event).collect(),
        )?;

        let command = DepositCommand { amount };

        let events = command.execute(account.clone())?;

        for event in events {
            event.apply(&mut account)?;

            self.event_store.append_event(
                account.account_id.ok_or_else(|| {
                    AccountServiceError::OperationError(
                        "Account ID is required for deposit event".to_string(),
                    )
                })?,
                ACCOUNT_AGGREGATE_TYPE,
                event.clone(),
            )?;

            self.event_bus.produce_event(event)?;
        }

        Ok(())
    }

    pub fn withdraw(&self, account_id: Ulid, amount: Decimal) -> Result<(), AccountServiceError> {
        let events_envelopes = self
            .event_store
            .get_events_for_aggregate(account_id, ACCOUNT_AGGREGATE_TYPE)?;
        let mut account = Account::from_history::<AccountEvent>(
            events_envelopes.into_iter().map(|e| e.event).collect(),
        )?;

        let command = WithdrawCommand { amount };

        let events = command.execute(account.clone())?;

        for event in events {
            event.apply(&mut account)?;

            self.event_store.append_event(
                account.account_id.ok_or_else(|| {
                    AccountServiceError::OperationError(
                        "Account ID is required for withdraw event".to_string(),
                    )
                })?,
                ACCOUNT_AGGREGATE_TYPE,
                event.clone(),
            )?;

            self.event_bus.produce_event(event)?;
        }

        Ok(())
    }

    pub fn get_account(&self, account_id: Ulid) -> Result<Account, AccountServiceError> {
        self.repository.get(account_id).map_err(Into::into)
    }
}
