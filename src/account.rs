pub mod account_handler;
pub mod account_service;
pub mod commands;
pub mod events;
pub mod repositories;

pub use account_handler::AccountHandler;
pub use account_service::AccountService;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use ulid::Ulid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Account {
    pub account_id: Option<Ulid>,
    pub balance: Decimal,
}

impl Account {
    #[tracing::instrument(level = "debug")]
    pub fn new(account_id: Option<Ulid>, balance: Decimal) -> Self {
        Self {
            account_id,
            balance,
        }
    }
}

impl Default for Account {
    fn default() -> Self {
        Self {
            account_id: None,
            balance: Decimal::from(0),
        }
    }
}
