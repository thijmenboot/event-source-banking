use rust_decimal::Decimal;
use thiserror::Error;
use ulid::Ulid;

use crate::{
    account::{Account, events::AccountOpenedEvent},
    traits::Command,
};

#[derive(Debug, Error)]
pub enum OpenAccountError {
    // No variants defined yet, can be added if specific errors arise
}

pub struct OpenAccountCommand {
    pub balance: Decimal,
}

impl Command<Account, AccountOpenedEvent, OpenAccountError> for OpenAccountCommand {
    fn execute(&self, _: Account) -> Result<Vec<AccountOpenedEvent>, OpenAccountError> {
        Ok(vec![AccountOpenedEvent {
            account_id: Ulid::new(),
            balance: self.balance,
        }])
    }
}
