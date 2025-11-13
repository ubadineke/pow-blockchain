use node::{State, Tx, error::StateError};

use crate::{AddArgs, TxSubcommand};

pub fn manage_txs(command: TxSubcommand) -> Result<(), StateError> {
    match command {
        TxSubcommand::Add(args) => {
            let AddArgs {
                from,
                to,
                value,
                data,
            } = args;

            //Add to In-Memory Mempool
            let mut state = State::new_from_disk().unwrap();
            let tx = Tx::new(from, to, value, data.unwrap_or("".to_string()));
            state.add(tx)?;

            //Flush to Disk
            state.persist()?;

            println!("TX successfully added to ledger");
            Ok(())
        }
    }
}
