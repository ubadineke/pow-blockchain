use node::{State, Tx};

use crate::{AddArgs, TxCommands};
pub fn manage_txs(command: TxCommands) {
    match command {
        TxCommands::Add(args) => {
            let AddArgs {
                from,
                to,
                value,
                data,
            } = args;

            //Add to In-Memory Mempool
            let mut state = State::new_from_disk().unwrap();
            let tx = Tx::new(from, to, value, data.unwrap_or("".to_string()));
            state.add(tx);

            //Flush to Disk
            state.persist();

            println!("TX successfully added to ledger");
        }
    }
}
