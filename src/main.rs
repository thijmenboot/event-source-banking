pub mod traits;
pub mod account;

use rust_decimal::Decimal;
use ulid::Ulid;

use account::{Account, events::{AccountEvent, AccountOpenedEvent, DepositEvent, WithdrawEvent}, commands::{OpenAccountCommand, DepositCommand, WithdrawCommand} };
use traits::{Command, Event, Aggregate};

fn main() {
    let account_id = Ulid::new();
    let history: Vec<AccountEvent> = vec![
        AccountEvent::Opened(AccountOpenedEvent {
            account_id,
            balance: Decimal::from(100),
        }),
        AccountEvent::Deposited(DepositEvent {
            account_id,
            amount: Decimal::from(50),
        }),
        AccountEvent::Withdrawn(WithdrawEvent {
            account_id,
            amount: Decimal::from(25),
        }),
    ];

    // balance at this time: 125
    let mut account = Account::from_history(history).expect("Failed to create account from event history");

    // Deposit 100
    let command = DepositCommand {
        amount: Decimal::from(100),
    };

    // Execute command
    let events = command.execute(account.clone()).expect("Failed to execute command");

    // Apply events
    for event in events {
        account = event.apply(account).expect("Failed to handle event");
    }

    // Balance at this time: 225
    println!("Account: {:?}", account);
}
