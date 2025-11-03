use std::{
    cell::RefCell,
    collections::{HashMap, VecDeque},
    env::current_dir,
    error::Error,
    fs::{self, File, OpenOptions},
    io::{BufRead, BufReader, BufWriter, Read, Seek, SeekFrom, Write},
    rc::Rc,
};

use serde::{Deserialize, Serialize};
use sha2::{Sha256, Digest};


#[derive(Hash, Eq, PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct Account(pub String);

#[derive(Serialize, Deserialize, Debug)]
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

#[derive(Hash, Eq, PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct Hash(pub [u8; 32]);

impl AsRef<[u8]> for Hash{
    fn as_ref(&self) -> &[u8] {
        &self.0 
    }
}

impl From<[u8; 32]> for Hash{
    fn from(value: [u8; 32]) -> Self {
        Hash(value)
    }
}
pub struct State {
    pub balances: HashMap<Account, u64>,
    pub tx_mempool: Rc<RefCell<VecDeque<Tx>>>,
    pub db_file: File,
    pub snapshot: Hash, //change to 32 byte array later
}

#[derive(Deserialize)]
pub struct Genesis {
    // genesis_time: String,
    // chain_id: String,
    balances: HashMap<String, u64>,
}

impl State {
    pub fn add(&mut self, tx: Tx) {
        self.apply(&tx).unwrap();
        self.tx_mempool.borrow_mut().push_back(tx);
    }

    pub fn apply(&mut self, tx: &Tx) -> Result<(), String> {
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
        if !self.balances.contains_key(&tx.to){
            self.balances.insert(tx.to.clone(), 0);
        }

        //Effect change
        *self.balances.get_mut(&tx.to).unwrap() += tx.value; //handle error here, if the account does not exist, throw Error

        // for (key, value) in &self.balances {
        //     // println!("Checking things{:?}, {}", key, value);
        //     balances.insert(Account(key), value);
        // }
        Ok(())
    }

    pub fn new_from_disk() -> Result<State, Box<dyn Error>> {
        //get current working directory
        let cwd = current_dir().expect("Failed to get current directory");
        let gen_file_path = cwd.join("database").join("genesis.json");

        let gen_data = fs::read_to_string(gen_file_path)?;
        // println!("{}", gen_data);
        let genesis: Genesis = serde_json::from_str(&gen_data)?;

        let mut balances: HashMap<Account, u64> = HashMap::new();

        for (key, value) in genesis.balances {
            // println!("{}, {}", key, value);
            balances.insert(Account(key), value);
        }

        let tx_db_file_path = current_dir()
            .expect("Failed to get current directory")
            .join("database")
            .join("tx.db");
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
            snapshot: Hash::from([0u8; 32]),
        };

        for line_result in reader.lines() {
            let line = line_result?;
            let tx: Tx = serde_json::from_str(&line)?;

            //Apply transaction to rebuild the balances
            state.apply(&tx)?;
        }

        Ok(state)
    }

    /// ADD/FLUSH TO DISK
    pub fn persist(&mut self) -> &Hash {
        //Add in this block, so the memory is freed from the **borrow_mut** and can be used forward in **self.snapshot**
        {
            let mut mempool = self.tx_mempool.borrow_mut();
            let mut writer = BufWriter::new(&self.db_file);

            while let Some(tx) = mempool.pop_front() {
                let tx_json = serde_json::to_string(&tx).unwrap();
                writeln!(writer, "{}", tx_json).unwrap();
            }
            writer.flush().unwrap();
        }
        //Process snapshot
        self.snapshot();
        println!("New DB Snapshot: {}", self.snapshot_to_hex());
        &self.snapshot
    }

    // Create a snapshot by hashing
    pub fn snapshot(&mut self) {
        let mut file_clone = &self.db_file;
        file_clone.seek(SeekFrom::Start(0)).unwrap();

        let mut reader = BufReader::new(file_clone);
        let mut string = String::new();
        reader.read_to_string(&mut string).unwrap();

        // let snapshot = digest(&string);
        let snapshot = State::compute_hash(&string.as_bytes());
        self.snapshot = Hash(snapshot);
    }

    pub fn snapshot_to_hex(&self) -> String{
        hex::encode(&self.snapshot)
    }

    pub fn hex_to_hash(hex_str: &str) -> [u8; 32] {
        let bytes = hex::decode(hex_str).expect("Invalid hex");
        bytes.try_into().expect("Wrong length")
    }

    fn compute_hash(data: &[u8]) -> [u8; 32] {
        let mut hasher = Sha256::new();
        hasher.update(data);
        hasher.finalize().into() 
    }
}
