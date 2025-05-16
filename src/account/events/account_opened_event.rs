use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

use ulid::Ulid;

use crate::{account::Account, traits::Event};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountOpenedEvent {
    pub account_id: Ulid,
    pub balance: Decimal,
}

impl Event<Account> for AccountOpenedEvent {
    fn apply(&self, _: Account) -> Result<Account, String> {
        Ok(Account {
            account_id: Some(self.account_id),
            balance: self.balance,
        })
    }

    fn aggregate_id(&self) -> Ulid {
        self.account_id
    }

    fn aggregate_type(&self) -> &str {
        "account"
    }

    fn event_type(&self) -> &str {
        "account_opened"
    }
}