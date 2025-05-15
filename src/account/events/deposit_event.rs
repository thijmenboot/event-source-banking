use rust_decimal::Decimal;
use ulid::Ulid;

use crate::{account::Account, traits::Event};

pub struct DepositEvent {
    pub account_id: Ulid,
    pub amount: Decimal,
}

impl Event<Account> for DepositEvent {
    fn apply(&self, state: Account) -> Result<Account, String> {
        Ok(Account {
            account_id: state.account_id,
            balance: state.balance + self.amount,
        })
    }
}