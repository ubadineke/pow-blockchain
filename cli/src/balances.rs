use node::{State, error::StateError};

use crate::BalancesSubcommand;
pub fn manage_balances(command: BalancesSubcommand) -> Result<(), StateError> {
    match command {
        BalancesSubcommand::List => {
            let state = State::new_from_disk().unwrap();
            println!("Account balances at {} \n", state.latest_blockhash.to_hex());
            println!("BALANCES:");
            println!("_________________");
            for (key, value) in state.balances {
                println!("{}: {}", key.0, value)
            }
            Ok(())
        }
    }
}
