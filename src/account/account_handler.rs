use crate::account::Account;
use crate::traits::event_bus::EventBusError;
use crate::traits::event_store::EventStoreError;
use crate::traits::{
    Aggregate, Event, EventBus, EventStore, Repository, event::ApplyError,
    repository::RepositoryError,
};

use super::events::{
    ACCOUNT_AGGREGATE_TYPE, AccountEvent, AccountOpenedEvent, DepositEvent, WithdrawEvent,
};

use thiserror::Error;

#[derive(Debug, Error)]
pub enum AccountHandlerError {
    #[error("Account handler error: {0}")]
    AccountHandlerError(String),

    #[error("Apply error: {0}")]
    ApplyError(#[from] ApplyError),

    #[error("Repository error: {0}")]
    RepositoryError(#[from] RepositoryError),

    #[error("Event bus error: {0}")]
    EventBusError(#[from] EventBusError),

    #[error("Event store error: {0}")]
    EventStoreError(#[from] EventStoreError),
}

pub struct AccountHandler<
    R: Repository<Account> + Send + Sync + Clone + 'static,
    B: EventBus + Send + Sync + Clone + 'static,
    S: EventStore + Send + Sync + Clone + 'static,
> {
    repository: R,
    event_bus: B,
    event_store: S,
}

impl<
    R: Repository<Account> + Send + Sync + Clone + 'static,
    B: EventBus + Send + Sync + Clone + 'static,
    S: EventStore + Send + Sync + Clone + 'static,
> AccountHandler<R, B, S>
{
    pub fn new(repository: R, event_bus: B, event_store: S) -> Self {
        Self {
            repository,
            event_bus,
            event_store,
        }
    }

    pub fn listen(&self) {
        let repository = self.repository.clone();
        let event_bus = self.event_bus.clone();
        let event_store = self.event_store.clone();

        self.event_bus.subscribe(
            "account",
            Box::new(move |event: AccountEvent| {
                let handler =
                    AccountHandler::new(repository.clone(), event_bus.clone(), event_store.clone());
                match event {
                    AccountEvent::Opened(event) => handler
                        .handle_account_opened(event)
                        .map_err(|e| EventBusError::HandleError(e.to_string())),
                    AccountEvent::Deposited(event) => handler
                        .handle_account_deposited(event)
                        .map_err(|e| EventBusError::HandleError(e.to_string())),
                    AccountEvent::Withdrawn(event) => handler
                        .handle_account_withdrawn(event)
                        .map_err(|e| EventBusError::HandleError(e.to_string())),
                }
            }),
        );
    }

    pub fn handle_account_opened(
        &self,
        event: AccountOpenedEvent,
    ) -> Result<(), AccountHandlerError> {
        let account = Account::from_history(vec![event])?;

        self.repository.create(account)?;
        Ok(())
    }

    pub fn handle_account_deposited(&self, event: DepositEvent) -> Result<(), AccountHandlerError> {
        let events_envelopes = self
            .event_store
            .get_events_for_aggregate(event.aggregate_id(), ACCOUNT_AGGREGATE_TYPE)?;
        let events: Vec<AccountEvent> = events_envelopes.into_iter().map(|e| e.event).collect();

        let account = Account::from_history(events)?;

        self.repository.update(account)?;
        Ok(())
    }

    pub fn handle_account_withdrawn(
        &self,
        event: WithdrawEvent,
    ) -> Result<(), AccountHandlerError> {
        let events_envelopes = self
            .event_store
            .get_events_for_aggregate(event.aggregate_id(), ACCOUNT_AGGREGATE_TYPE)?;
        let events: Vec<AccountEvent> = events_envelopes.into_iter().map(|e| e.event).collect();

        let account = Account::from_history(events)?;

        self.repository.update(account)?;
        Ok(())
    }
}
