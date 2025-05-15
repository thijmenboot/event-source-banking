use rust_decimal::Decimal;
use ulid::Ulid;

use crate::{account::{events::AccountOpenedEvent, Account}, traits::Command};

pub struct OpenAccountCommand {
    pub balance: Decimal,
}

impl Command<Account, AccountOpenedEvent> for OpenAccountCommand {
    fn execute(&self, _: Account) -> Result<Vec<AccountOpenedEvent>, String> {
        Ok(vec![AccountOpenedEvent {
            account_id: Ulid::new(),
            balance: self.balance,
        }])
    }
}