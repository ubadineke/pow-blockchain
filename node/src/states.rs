use std::{
    cell::RefCell,
    collections::{HashMap, VecDeque},
    env::current_dir,
    fs::{self, File, OpenOptions},
    io::{BufRead, BufReader, BufWriter, Write},
    rc::Rc,
};

use serde::{Deserialize, Serialize};

use crate::{
    block::{Block, BlockRecord},
    error::{GenesisError, StateError},
};

#[derive(Hash, Eq, PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct Account(pub String);

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Tx {
    pub from: Account,
    pub to: Account,
    pub value: u64,
    pub data: String,
}

impl Tx {
    pub fn is_reward(&self) -> bool {
        self.data == "reward".to_string()
    }

    pub fn new(from: String, to: String, value: u64, data: String) -> Tx {
        Self {
            from: Account(from),
            to: Account(to),
            value,
            data,
        }
    }
}

#[derive(Serialize, Deserialize, Copy, Clone, Debug)]
pub struct Hash(pub [u8; 32]);

impl AsRef<[u8]> for Hash {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

impl From<[u8; 32]> for Hash {
    fn from(value: [u8; 32]) -> Self {
        Hash(value)
    }
}

impl Hash {
    pub fn to_hex(&self) -> String {
        hex::encode(&self.0)
    }
}
pub struct State {
    pub balances: HashMap<Account, u64>,
    pub tx_mempool: Rc<RefCell<VecDeque<Tx>>>,
    pub db_file: File,
    pub latest_blockhash: Hash,
}

#[derive(Deserialize)]
pub struct Genesis {
    pub genesis_time: String,
    pub chain_id: String,
    pub balances: HashMap<String, u64>,
}

impl Genesis {
    pub fn new() -> Result<Self, GenesisError> {
        let cwd = current_dir()?;
        let gen_file_path = cwd.join("database/src/genesis.json");
        let gen_data = fs::read_to_string(gen_file_path)?;

        let genesis: Genesis = serde_json::from_str(&gen_data)?;
        Ok(genesis)
    }
}

impl State {
    pub fn add(&mut self, tx: Tx) -> Result<(), StateError> {
        self.apply(&tx, self.latest_blockhash)?;
        self.tx_mempool.borrow_mut().push_back(tx);
        Ok(())
    }

    pub fn apply(&mut self, tx: &Tx, block_hash: Hash) -> Result<(), StateError> {
        if tx.is_reward() {
            // println!("is_reward_yes");
            *self.balances.entry(tx.to.clone()).or_insert(0) += tx.value;
        }

        //Check if 'from' account has sufficient balance
        let from_balance = self.balances.get(&tx.from).copied().unwrap_or(0);
        if tx.value > from_balance {
            // return Err("insufficient value".to_string());
            return Err(StateError::InsufficientBalance {
                account: tx.from.clone().0,
                requested: tx.value,
                available: from_balance,
            });
        }

        //Effect change
        *self
            .balances
            .get_mut(&tx.from)
            .expect("from account missing") -= tx.value; //don't expect this to fail though since the Tx would always exist

        //If the 'to' account does not exist, create it with initial balance 0(the .or_insert(0) does that)
        //Effect change
        *self.balances.entry(tx.to.clone()).or_insert(0) += tx.value; //handle error here, if the account does not exist, throw Error

        self.latest_blockhash = block_hash;
        Ok(())
    }

    pub fn new_from_disk() -> Result<State, StateError> {
        //get current working directory
        let cwd = current_dir()?;
        let gen_file_path = cwd.join("database/src/genesis.json");

        let gen_data = fs::read_to_string(gen_file_path)?;
        let genesis: Genesis = serde_json::from_str(&gen_data)?;

        let mut balances: HashMap<Account, u64> = HashMap::new();

        for (key, value) in genesis.balances {
            // println!("{}, {}", key, value);
            balances.insert(Account(key), value);
        }

        let tx_db_file_path = current_dir()?.join("database/src/blocks.db");
        let file = OpenOptions::new()
            .read(true)
            .append(true)
            .open(tx_db_file_path)?;
        let reader = BufReader::new(file.try_clone()?);

        //initialize state
        let mut state = State {
            balances,
            tx_mempool: Rc::new(RefCell::new(VecDeque::new())),
            db_file: file,
            latest_blockhash: Hash::from([0u8; 32]),
        };

        for line_result in reader.lines() {
            let line = line_result?;
            let block_record: BlockRecord = serde_json::from_str(&line)?;
            //Apply transaction to rebuild the balances
            for tx in &block_record.block.txs {
                state.apply(&tx, State::hex_to_hash(&block_record.blockhash)?)?;
            }
        }

        Ok(state)
    }

    /// ADD/FLUSH TO DISK
    pub fn persist(&mut self) -> Result<Hash, StateError> {
        let block = Block::new(self.latest_blockhash, self.tx_mempool.borrow().clone());
        let blockhash = block.hash();

        let block_record = BlockRecord {
            blockhash: blockhash.to_hex(),
            block,
        };
        let block_json = serde_json::to_string(&block_record)?;

        let mut writer = BufWriter::new(&self.db_file);
        writeln!(writer, "{}", block_json)?;

        self.latest_blockhash = blockhash;
        self.tx_mempool = Rc::new(RefCell::new(VecDeque::new()));

        Ok(blockhash)
    }

    pub fn hash_to_hex(&self) -> String {
        hex::encode(&self.latest_blockhash)
    }

    pub fn hex_to_hash(hex_str: &str) -> Result<Hash, StateError> {
        let bytes = hex::decode(hex_str)?;
        let arr: [u8; 32] = bytes
            .as_slice()
            .try_into()
            .map_err(|_| StateError::InvalidLength(bytes.len()))?;
        Ok(Hash(arr))
    }

    pub fn add_block(&mut self, block: Block) -> Result<(), StateError> {
        for tx in block.txs {
            self.add(tx)?;
        }
        Ok(())
    }
}
