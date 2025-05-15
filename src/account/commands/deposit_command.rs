use rust_decimal::Decimal;
use ulid::Ulid;

use crate::{account::{Account, events::DepositEvent}, traits::Command};

pub struct DepositCommand {
    pub amount: Decimal,
}

impl Command<Account, DepositEvent> for DepositCommand {
    fn execute(&self, _: Account) -> Result<Vec<DepositEvent>, String> {
        Ok(vec![DepositEvent {
            account_id: Ulid::new(),
            amount: self.amount,
        }])
    }
}