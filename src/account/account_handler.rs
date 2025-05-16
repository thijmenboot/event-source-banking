use crate::traits::{Aggregate, Event, EventBus, Repository, EventStore};
use crate::account::Account;

use super::events::{AccountEvent, AccountOpenedEvent, DepositEvent, WithdrawEvent, ACCOUNT_AGGREGATE_TYPE};

pub struct AccountHandler<R: Repository<Account> + Send + Sync + Clone + 'static, 
                         B: EventBus + Send + Sync + Clone + 'static, 
                         S: EventStore + Send + Sync + Clone + 'static> {
    repository: R,
    event_bus: B,
    event_store: S,
}

impl<R: Repository<Account> + Send + Sync + Clone + 'static, 
     B: EventBus + Send + Sync + Clone + 'static, 
     S: EventStore + Send + Sync + Clone + 'static> AccountHandler<R, B, S> {
    pub fn new(repository: R, event_bus: B, event_store: S) -> Self {
        Self { repository, event_bus, event_store }
    }

    pub fn listen(&self) {
        let repository = self.repository.clone();
        let event_bus = self.event_bus.clone();
        let event_store = self.event_store.clone();
        
        self.event_bus.subscribe("account", Box::new(move |event: AccountEvent| {
            let handler = AccountHandler::new(repository.clone(), event_bus.clone(), event_store.clone());
            match event {
                AccountEvent::Opened(event) => handler.handle_account_opened(event),
                AccountEvent::Deposited(event) => handler.handle_account_deposited(event),
                AccountEvent::Withdrawn(event) => handler.handle_account_withdrawn(event),
            }
        }));
    }

    pub fn handle_account_opened(&self, event: AccountOpenedEvent) -> Result<(), String> {
        let account = Account::from_history(vec![event])?;

        self.repository.create(account)
    }

    pub fn handle_account_deposited(&self, event: DepositEvent) -> Result<(), String> {
        let events = self.event_store.get_events_for_aggregate(event.aggregate_id(), ACCOUNT_AGGREGATE_TYPE)?;
        let events: Vec<AccountEvent> = events.into_iter().map(|e| e.event).collect();
        
        let account = Account::from_history(events)?;

        self.repository.update(account)
    }

    pub fn handle_account_withdrawn(&self, event: WithdrawEvent) -> Result<(), String> {
        let events = self.event_store.get_events_for_aggregate(event.aggregate_id(), ACCOUNT_AGGREGATE_TYPE)?;
        let events: Vec<AccountEvent> = events.into_iter().map(|e| e.event).collect();
        
        let account = Account::from_history(events)?;

        self.repository.update(account)
    }
}
