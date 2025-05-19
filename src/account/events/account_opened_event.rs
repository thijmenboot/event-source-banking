use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

use ulid::Ulid;

use crate::{account::Account, traits::Event, traits::event::ApplyError};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountOpenedEvent {
    pub account_id: Ulid,
    pub balance: Decimal,
}

impl Event<Account> for AccountOpenedEvent {
    fn apply(&self, account: &mut Account) -> Result<(), ApplyError> {
        account.account_id = Some(self.account_id);
        account.balance = self.balance;
        Ok(())
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
