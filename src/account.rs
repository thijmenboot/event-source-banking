pub mod events;
pub mod commands;

use rust_decimal::Decimal;
use ulid::Ulid;

#[derive(Debug, Clone)]
pub struct Account {
    pub account_id: Option<Ulid>,
    pub balance: Decimal,
}

impl Default for Account {
    fn default() -> Self {
        Self { account_id: None, balance: Decimal::from(0) }
    }
}