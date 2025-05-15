use rusqlite::Connection;
use rusqlite::named_params;
use std::str::FromStr;

use crate::{account::Account, traits::Repository};

pub struct AccountRepositorySqlite {
    db: Connection,
}

impl AccountRepositorySqlite {
    pub fn new(db_path: &str) -> Self {
        Self { db: Connection::open(db_path).expect("Failed to open database") }
    }
}

impl Repository<Account> for AccountRepositorySqlite {
    
    fn create(&self, aggregate: Account) -> Result<(), String> {
        let account_id = aggregate.account_id.expect("Account ID is required");
        let balance = aggregate.balance;

        let mut statement = self.db.prepare("INSERT INTO accounts (account_id, balance) VALUES (?, ?)").expect("Failed to prepare statement");
        
        statement.execute(named_params! {
            ":id": account_id.to_string(),
            ":balance": balance.to_string(),
        }).expect("Failed to execute statement");

        Ok(())
    }
    
    fn update(&self, aggregate: Account) -> Result<(), String> {
        let account_id = aggregate.account_id.expect("Account ID is required");
        let balance = aggregate.balance;

        let mut statement = self.db.prepare("UPDATE accounts SET balance = ? WHERE account_id = ?").expect("Failed to prepare statement");
        
        statement.execute(named_params! {
            ":id": account_id.to_string(),
            ":balance": balance.to_string(),
        }).expect("Failed to execute statement");

        Ok(())
    }
    
    fn delete(&self, id: ulid::Ulid) -> Result<(), String> {
        let mut statement = self.db.prepare("DELETE FROM accounts WHERE account_id = ?").expect("Failed to prepare statement");
        
        statement.execute(named_params! {
            ":id": id.to_string(),
        }).expect("Failed to execute statement");

        Ok(())
    }
    
    fn get(&self, id: ulid::Ulid) -> Result<Account, String> {
        let mut statement = self.db.prepare("SELECT account_id, balance FROM accounts WHERE account_id = :id")
            .expect("Failed to prepare statement");
        
        let account = statement.query_row(named_params! {
            ":id": id.to_string()
        }, |row| {
            let account_id = row.get::<_, String>(0)
                .and_then(|s| ulid::Ulid::from_string(&s).map_err(|e| rusqlite::Error::FromSqlConversionFailure(0, rusqlite::types::Type::Text, Box::new(e))))
                .expect("Failed to parse ULID");
            
            let balance = row.get::<_, String>(1)
                .and_then(|s| rust_decimal::Decimal::from_str(&s).map_err(|e| rusqlite::Error::FromSqlConversionFailure(1, rusqlite::types::Type::Text, Box::new(e))))
                .expect("Failed to parse Decimal");

            Ok(Account {
                account_id: Some(account_id),
                balance,
            })
        }).expect("Failed to query row");

        Ok(account)
    }
}