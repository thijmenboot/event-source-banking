use rust_decimal::Decimal;
use ulid::Ulid;

use crate::{account::{Account, events::DepositEvent}, traits::Command};

pub struct DepositCommand {
    pub amount: Decimal,
}

impl Command<Account, DepositEvent> for DepositCommand {
    fn execute(&self, account: Account) -> Result<Vec<DepositEvent>, String> {
        Ok(vec![DepositEvent {
            account_id: account.account_id.ok_or("Account ID is required")?,
            amount: self.amount,
        }])
    }
}