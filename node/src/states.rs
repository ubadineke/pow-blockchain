use std::{
    cell::RefCell,
    collections::{HashMap, VecDeque},
    env::current_dir,
    error::Error,
    fs::{self, File, OpenOptions},
    io::{BufRead, BufReader, BufWriter, Write},
    rc::Rc,
};

use serde::{Deserialize, Serialize};

use crate::block::{Block, BlockRecord};

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
    pub fn new() -> Self {
        let cwd = current_dir().expect("Failed to get current directory");
        let gen_file_path = cwd.join("database/src/genesis.json");
        let gen_data = fs::read_to_string(gen_file_path).unwrap();

        let Genesis {
            genesis_time,
            chain_id,
            balances,
        } = serde_json::from_str(&gen_data).unwrap();
        Self {
            genesis_time,
            chain_id,
            balances,
        }
    }
}

impl State {
    pub fn add(&mut self, tx: Tx) {
        self.apply(&tx, self.latest_blockhash).unwrap();
        self.tx_mempool.borrow_mut().push_back(tx);
    }

    pub fn apply(&mut self, tx: &Tx, block_hash: Hash) -> Result<(), String> {
        if tx.is_reward() {
            // println!("is_reward_yes");
            *self.balances.entry(tx.to.clone()).or_insert(0) += tx.value;
        }

        //Check if 'from' account has sufficient balance
        if tx.value > *self.balances.get(&tx.from).unwrap_or(&0) {
            return Err("insufficient value".to_string());
        }

        //Effect change
        *self.balances.get_mut(&tx.from).unwrap() -= tx.value; //handle error here, if the account does not exist, throw Error

        //If the 'to' account does not exist, create it with initial balance 0
        if !self.balances.contains_key(&tx.to) {
            self.balances.insert(tx.to.clone(), 0);
        }

        //Effect change
        *self.balances.get_mut(&tx.to).unwrap() += tx.value; //handle error here, if the account does not exist, throw Error

        self.latest_blockhash = block_hash;
        Ok(())
    }

    pub fn new_from_disk() -> Result<State, Box<dyn Error>> {
        //get current working directory
        let cwd = current_dir().expect("Failed to get current directory");
        let gen_file_path = cwd.join("database/src/genesis.json");

        let gen_data = fs::read_to_string(gen_file_path)?;
        let genesis: Genesis = serde_json::from_str(&gen_data)?;

        let mut balances: HashMap<Account, u64> = HashMap::new();

        for (key, value) in genesis.balances {
            // println!("{}, {}", key, value);
            balances.insert(Account(key), value);
        }

        let tx_db_file_path = current_dir()
            .expect("Failed to get current directory")
            .join("database/src/blocks.db");
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
            let line = line_result.unwrap();
            let block_record: BlockRecord = serde_json::from_str(&line).unwrap();
            //Apply transaction to rebuild the balances
            for tx in &block_record.block.txs {
                state.apply(&tx, State::hex_to_hash(&block_record.blockhash))?;
            }
        }

        Ok(state)
    }

    /// ADD/FLUSH TO DISK
    pub fn persist(&mut self) -> Hash {
        let block = Block::new(self.latest_blockhash, self.tx_mempool.borrow().clone());
        let blockhash = block.hash();

        let block_record = BlockRecord {
            blockhash: blockhash.to_hex(),
            block,
        };
        let block_json = serde_json::to_string(&block_record).unwrap();

        let mut writer = BufWriter::new(&self.db_file);
        writeln!(writer, "{}", block_json).unwrap();

        self.latest_blockhash = blockhash;
        self.tx_mempool = Rc::new(RefCell::new(VecDeque::new()));

        blockhash
    }

    pub fn hash_to_hex(&self) -> String {
        hex::encode(&self.latest_blockhash)
    }

    pub fn hex_to_hash(hex_str: &str) -> Hash {
        let bytes = hex::decode(hex_str).expect("Invalid hex");
        let arr: [u8; 32] = bytes.as_slice().try_into().expect("Expected 32 bytes");
        Hash(arr)
    }

    pub fn add_block(&mut self, block: Block) {
        for tx in block.txs {
            self.add(tx);
        }
    }
}
