use std::{
    collections::VecDeque,
    time::{SystemTime, UNIX_EPOCH},
};

use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

use crate::{Hash, Tx};

#[derive(Serialize, Deserialize, Debug)]
pub struct BlockHeader {
    pub parent: Hash,
    pub time: u64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Block {
    pub header: BlockHeader,
    #[serde(rename = "payload")]
    pub txs: VecDeque<Tx>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BlockRecord {
    #[serde(rename = "hash")]
    pub blockhash: String,
    pub block: Block,
}

impl Block {
    pub fn new(latest_blockhash: Hash, txs: VecDeque<Tx>) -> Self {
        let time = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
        let unix_timestamp = time.as_secs();

        let header = BlockHeader {
            parent: latest_blockhash,
            time: unix_timestamp,
        };
        Self { header, txs }
    }

    pub fn hash(&self) -> Hash {
        let block_json = serde_json::to_string(&self).unwrap();
        let bytes = Block::compute_hash(&block_json.as_bytes());
        Hash::from(bytes)
    }

    fn compute_hash(data: &[u8]) -> [u8; 32] {
        let mut hasher = Sha256::new();
        hasher.update(data);
        hasher.finalize().into()
    }
}
