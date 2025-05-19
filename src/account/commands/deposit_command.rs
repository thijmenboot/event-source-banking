use rust_decimal::Decimal;
use thiserror::Error;

use crate::{
    account::{Account, events::DepositEvent},
    traits::Command,
};

#[derive(Debug, Error)]
pub enum DepositError {
    #[error("Account ID is required: {0}")]
    AccountIdMissing(String),
}

pub struct DepositCommand {
    pub amount: Decimal,
}

impl Command<Account, DepositEvent, DepositError> for DepositCommand {
    fn execute(&self, account: Account) -> Result<Vec<DepositEvent>, DepositError> {
        Ok(vec![DepositEvent {
            account_id: account.account_id.ok_or_else(|| {
                DepositError::AccountIdMissing("Account ID is required for deposit".to_string())
            })?,
            amount: self.amount,
        }])
    }
}
