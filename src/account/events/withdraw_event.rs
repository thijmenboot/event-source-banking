use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use ulid::Ulid;

use crate::{account::Account, traits::Event};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WithdrawEvent {
    pub account_id: Ulid,
    pub amount: Decimal,
}

impl Event<Account> for WithdrawEvent {
    fn apply(&self, state: Account) -> Result<Account, String> {
        let new_balance = state.balance - self.amount;

        if new_balance < Decimal::from(0) {
            return Err("Insufficient balance".to_string());
        }

        Ok(Account {
            account_id: state.account_id,
            balance: new_balance,
        })
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