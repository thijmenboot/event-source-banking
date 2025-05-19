use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use ulid::Ulid;

use crate::{account::Account, traits::Event, traits::event::ApplyError};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WithdrawEvent {
    pub account_id: Ulid,
    pub amount: Decimal,
}

impl Event<Account> for WithdrawEvent {
    fn apply(&self, state: &mut Account) -> Result<(), ApplyError> {
        let new_balance = state.balance - self.amount;

        if new_balance < Decimal::from(0) {
            return Err(ApplyError::InvariantViolated(
                "Insufficient balance".to_string(),
            ));
        }
        state.balance = new_balance;

        Ok(())
    }

    fn aggregate_id(&self) -> Ulid {
        self.account_id
    }

    fn aggregate_type(&self) -> &str {
        "account"
    }

    fn event_type(&self) -> &str {
        "withdraw"
    }
}
