use rust_decimal::Decimal;
use thiserror::Error;

use crate::{
    account::{Account, events::WithdrawEvent},
    traits::Command,
};

#[derive(Debug, Error)]
pub enum WithdrawError {
    #[error("Insufficient balance: {0}")]
    InsufficientBalance(String),
    #[error("Account not opened or ID missing: {0}")]
    AccountNotOpened(String),
}

pub struct WithdrawCommand {
    pub amount: Decimal,
}

impl Command<Account, WithdrawEvent, WithdrawError> for WithdrawCommand {
    fn execute(&self, state: Account) -> Result<Vec<WithdrawEvent>, WithdrawError> {
        if (state.balance - self.amount) < Decimal::from(0) {
            return Err(WithdrawError::InsufficientBalance(
                "Cannot withdraw an amount greater than the current balance.".to_string(),
            ));
        }

        Ok(vec![WithdrawEvent {
            account_id: state.account_id.ok_or_else(|| {
                WithdrawError::AccountNotOpened(
                    "Account ID is missing, cannot process withdrawal.".to_string(),
                )
            })?,
            amount: self.amount,
        }])
    }
}
