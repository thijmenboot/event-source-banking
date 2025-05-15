pub mod account_opened_event;
pub mod deposit_event;
pub mod withdraw_event;

pub use account_opened_event::AccountOpenedEvent;
pub use deposit_event::DepositEvent;
pub use withdraw_event::WithdrawEvent;

use crate::{traits::Event, Account};

// Define this where you have your event types
pub enum AccountEvent {
    Opened(AccountOpenedEvent),
    Deposited(DepositEvent),
    Withdrawn(WithdrawEvent),
}

impl Event<Account> for AccountEvent {
    fn apply(&self, state: Account) -> Result<Account, String> {
        match self {
            AccountEvent::Opened(e) => e.apply(state),
            AccountEvent::Deposited(e) => e.apply(state),
            AccountEvent::Withdrawn(e) => e.apply(state),
        }
    }
}