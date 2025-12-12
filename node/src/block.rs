use std::{
    collections::VecDeque,
    time::{SystemTime, UNIX_EPOCH},
};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use crate::{error::StateError, Hash, Tx};

#[derive(Serialize, Deserialize, Debug)]
pub struct BlockHeader {
    pub parent: Hash,
    pub height: u64,
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
    pub fn new(latest_blockhash: Hash, txs: VecDeque<Tx>, block_height: u64) -> Result<Self, StateError> {
        let time = SystemTime::now().duration_since(UNIX_EPOCH)?;
        let unix_timestamp = time.as_secs();

        let header = BlockHeader {
            parent: latest_blockhash,
            height: block_height,
            time: unix_timestamp,
        };
        Ok(Self { header, txs })
    }

    pub fn hash(&self) -> Result<Hash, StateError> {
        let block_json = serde_json::to_string(&self)?;
        let bytes = Block::compute_hash(&block_json.as_bytes());
        Ok(Hash::from(bytes))
    }

    fn compute_hash(data: &[u8]) -> [u8; 32] {
        let mut hasher = Sha256::new();
        hasher.update(data);
        hasher.finalize().into()
    }
}
