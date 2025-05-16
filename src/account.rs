pub mod events;
pub mod commands;
pub mod repositories;
pub mod account_service;
pub mod account_handler;

pub use account_service::AccountService;
pub use account_handler::AccountHandler;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use ulid::Ulid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Account {
    pub account_id: Option<Ulid>,
    pub balance: Decimal,
}

impl Default for Account {
    fn default() -> Self {
        Self { account_id: None, balance: Decimal::from(0) }
    }
}