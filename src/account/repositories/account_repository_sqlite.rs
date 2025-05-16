use rusqlite::named_params;
use std::str::FromStr;
use crate::{account::Account, traits::Repository};
use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;

#[derive(Debug, Clone)]
pub struct AccountRepositorySqlite {
    pool: Pool<SqliteConnectionManager>,
}

impl AccountRepositorySqlite {
    pub fn new(db_path: &str) -> Self {
        let manager = SqliteConnectionManager::file(db_path);
        let pool = Pool::new(manager).expect("Failed to create pool");
        
        // Apply migrations
        let conn = pool.get().expect("Failed to get connection");
        conn.execute_batch(
            "CREATE TABLE IF NOT EXISTS accounts (
                account_id TEXT PRIMARY KEY NOT NULL,
                balance TEXT NOT NULL,
                created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
                updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
            );"
        ).expect("Failed to create accounts table");
        
        Self { pool }
    }
}

impl Repository<Account> for AccountRepositorySqlite {
    
    fn create(&self, aggregate: Account) -> Result<(), String> {
        let account_id = aggregate.account_id.ok_or("Account ID is required")?;
        let balance = aggregate.balance;

        let conn = self.pool.get().map_err(|e| e.to_string())?;
        let mut statement = conn.prepare("INSERT INTO accounts (account_id, balance) VALUES (:account_id, :balance)")
            .map_err(|e| e.to_string())?;
        
        statement.execute(named_params! {
            ":account_id": account_id.to_string(),
            ":balance": balance.to_string(),
        }).map_err(|e| e.to_string())?;

        println!("Account created in projection: {:?}", account_id);

        Ok(())
    }
    
    fn update(&self, aggregate: Account) -> Result<(), String> {
        let account_id = aggregate.account_id.ok_or("Account ID is required")?;
        let balance = aggregate.balance;

        let conn = self.pool.get().map_err(|e| e.to_string())?;
        let mut statement = conn.prepare("UPDATE accounts SET balance = :balance WHERE account_id = :account_id")
            .map_err(|e| e.to_string())?;
        
        statement.execute(named_params! {
            ":account_id": account_id.to_string(),
            ":balance": balance.to_string(),
        }).map_err(|e| e.to_string())?;

        println!("Account ID {:?} balance updated in projection: {:?}", account_id, balance);

        Ok(())
    }
    
    fn delete(&self, id: ulid::Ulid) -> Result<(), String> {
        let conn = self.pool.get().map_err(|e| e.to_string())?;
        let mut statement = conn.prepare("DELETE FROM accounts WHERE account_id = :account_id")
            .map_err(|e| e.to_string())?;
        
        statement.execute(named_params! {
            ":account_id": id.to_string(),
        }).map_err(|e| e.to_string())?;

        println!("Account ID {:?} deleted from projection", id);

        Ok(())
    }
    
    fn get(&self, id: ulid::Ulid) -> Result<Account, String> {
        let conn = self.pool.get().map_err(|e| e.to_string())?;
        let mut statement = conn.prepare("SELECT account_id, balance FROM accounts WHERE account_id = :account_id")
            .map_err(|e| e.to_string())?;
        
        let account = statement.query_row(named_params! {
            ":account_id": id.to_string()
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
        }).map_err(|e| e.to_string())?;

        Ok(account)
    }
}