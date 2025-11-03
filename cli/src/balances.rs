use node::State;

use crate::BalancesCommands;
pub fn manage_balances(command: BalancesCommands) {
    match command {
        BalancesCommands::List => {
            let mut state = State::new_from_disk().unwrap();
            state.snapshot();
            println!("Account balances at {} \n", state.snapshot_to_hex());
            println!("BALANCES:");
            println!("_________________");
            for (key, value) in state.balances {
                println!("{}: {}", key.0, value)
            }
        }
    }
}
