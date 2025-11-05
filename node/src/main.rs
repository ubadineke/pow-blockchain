use std::collections::VecDeque;

use node::{Hash, State, Tx, block::Block};

fn main() {
    println!("Migrating Data from Databases.!");

    //SIMPLE SETUP FOR MIGRATING FROM TX.DB TO BLOCKS.DB
    let mut state = State::new_from_disk().unwrap();

    let block0 = Block::new(
        Hash([0u8; 32]),
        VecDeque::from(vec![
            Tx::new("clement".into(), "clement".into(), 3, "".into()),
            Tx::new("clement".into(), "clement".into(), 700, "reward".into()),
        ]),
    );
    state.add_block(block0);
    let block0_hash = state.persist();
    println!("Block 0 hash: {}", &block0_hash.to_hex());

    let block1 = Block::new(
      block0_hash,
      VecDeque::from(vec![
        Tx::new("clement".into(), "uba".into(), 2000, "".into()),
        Tx::new("clement".into(), "clement".into(), 100, "reward".into()),
        Tx::new("uba".into(), "clement".into(), 1, "".into()),
        Tx::new("uba".into(), "doms".into(), 1000, "".into()),
        Tx::new("uba".into(), "clement".into(), 50, "".into()),
        Tx::new("clement".into(), "clement".into(), 600, "reward".into()),
      ])
    );
    state.add_block(block1);
    let block1_hash = state.persist();
    println!("Block 1 hash: {}", &block1_hash.to_hex())

}
