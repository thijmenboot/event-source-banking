use rust_decimal::Decimal;
use ulid::Ulid;

use crate::{account::Account, traits::Event};


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
}