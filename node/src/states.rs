use std::{
    collections::HashMap,
    env::current_dir,
    error::Error,
    fs::{self, File, OpenOptions},
    io::{BufRead, BufReader},
};

use serde::Deserialize;

#[derive(Hash, Eq, PartialEq, Debug, Clone, Deserialize)]
pub struct Account(pub String);

#[derive(Deserialize, Debug)]
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
}

pub struct State {
    pub balances: HashMap<Account, u64>,
    pub tx_mempool: Vec<Tx>,
    pub db_file: File,
}

#[derive(Deserialize)]
pub struct Genesis {
    genesis_time: String,
    chain_id: String,
    balances: HashMap<String, u64>,
}

impl State {
    pub fn add(&mut self, tx: Tx) {
        self.apply(&tx).unwrap();
        self.tx_mempool.push(tx);
    }

    pub fn apply(&mut self, tx: &Tx) -> Result<(), String> {
        if tx.is_reward() {
            println!("is_reward_yes");
            *self.balances.entry(tx.to.clone()).or_insert(0) += tx.value;
        }

        //Check if 'from' account has sufficient balance
        if tx.value > *self.balances.get(&tx.from).unwrap_or(&0) {
            return Err("insufficient value".to_string());
        }

        //Effect change
        *self.balances.get_mut(&tx.from).unwrap() -= tx.value; //handle error here, if the account does not exist, throw Error
        // let vall = *self.balances.get_mut(&tx.from).unwrap();

        //If the 'to' account does not exist, create it with initial balance 0
        if (!self.balances.contains_key(&tx.to)){
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
            tx_mempool: Vec::new(),
            db_file: file,
        };

        for line_result in reader.lines() {
            let line = line_result?;
            let tx: Tx = serde_json::from_str(&line)?;

            //Apply transaction to rebuild the balances
            state.apply(&tx)?;
        }

        Ok(state)
    }
    
    pub fn persist(){

    }
}
