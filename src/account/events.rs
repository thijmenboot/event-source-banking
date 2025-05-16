pub mod account_opened_event;
pub mod deposit_event;
pub mod withdraw_event;

pub use account_opened_event::AccountOpenedEvent;
pub use deposit_event::DepositEvent;
use serde::{Deserialize, Serialize};
use ulid::Ulid;
pub use withdraw_event::WithdrawEvent;

use crate::{traits::Event, Account};

// Define this where you have your event types
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AccountEvent {
    Opened(AccountOpenedEvent),
    Deposited(DepositEvent),
    Withdrawn(WithdrawEvent),
}

pub const ACCOUNT_AGGREGATE_TYPE: &str = "account";

impl Event<Account> for AccountEvent {
    fn apply(&self, state: Account) -> Result<Account, String> {
        match self {
            AccountEvent::Opened(e) => e.apply(state),
            AccountEvent::Deposited(e) => e.apply(state),
            AccountEvent::Withdrawn(e) => e.apply(state),
        }
    }

    fn aggregate_id(&self) -> Ulid {
        match self {
            AccountEvent::Opened(e) => e.aggregate_id(),
            AccountEvent::Deposited(e) => e.aggregate_id(),
            AccountEvent::Withdrawn(e) => e.aggregate_id(),
        }
    }

    fn aggregate_type(&self) -> &str {
        ACCOUNT_AGGREGATE_TYPE
    }

    fn event_type(&self) -> &str {
        match self {
            AccountEvent::Opened(e) => e.event_type(),
            AccountEvent::Deposited(e) => e.event_type(),
            AccountEvent::Withdrawn(e) => e.event_type(),
        }
    }
}