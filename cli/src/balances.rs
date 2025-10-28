use node::State;

use crate::BalancesCommands;
pub fn manage_balances(command: BalancesCommands) {
  match command {
    BalancesCommands::List => {
        let state = State::new_from_disk().unwrap();
        println!("Account balances");
        println!("_________________");
        for (key, value) in state.balances {
          println!("{}: {}", key.0, value)
        }
    }
  }
}