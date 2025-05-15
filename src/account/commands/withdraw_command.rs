use rust_decimal::Decimal;

use crate::{account::{Account, events::WithdrawEvent}, traits::Command};

pub struct WithdrawCommand {
    pub amount: Decimal,
}

impl Command<Account, WithdrawEvent> for WithdrawCommand {
    fn execute(&self, state: Account) -> Result<Vec<WithdrawEvent>, String> {
        if (state.balance - self.amount) < Decimal::from(0) {
            return Err("Insufficient balance".to_string());
        }

        Ok(vec![WithdrawEvent {
            account_id: state.account_id.ok_or("Account not opened")?,
            amount: self.amount,
        }])
    }
}