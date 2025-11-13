use std::{
    env::{self, current_dir, home_dir},
    fs::{self, File},
    path::{Path, PathBuf},
};
pub struct DataDir {}

impl DataDir {
    pub fn init() -> PathBuf {
        //Intended Directory: /home/<username>/.ubachain/database

        //Use path specified in ENV if provided, else use default setup.
        let path = match env::var("DATA_DIR") {
            Ok(env_path) => {
                println!("Data Directory configured in ENV...");
                let path = Path::new(&env_path);
                //Create path if not existing
                if !path.exists() {
                    println!("Path provided in ENV not existing, creating ...");
                    fs::create_dir_all(path).unwrap();
                }
                DataDir::create_starter_files(&path); //add required Files
                path.to_path_buf()
            }
            Err(_) => {
                println!("Data Directory not in ENV, using default config...");

                //Check default path if data and return
                let home_dir = home_dir().expect("Failed to get home directory");
                let default_path = home_dir.join(".ubachain/database");
                if !default_path.exists() {
                    println!("Default path doesn't exist, creating...");
                    fs::create_dir_all(&default_path).unwrap();
                    DataDir::create_starter_files(&default_path); //add required Files
                }
                default_path
            }
        };
        path
    }

    fn create_starter_files(dir: &Path) {
        //genesis.json - copy/write from file in the repo.
        let gen_file_path = current_dir()
            .expect("Failed to get current directory")
            .join("database/src/genesis.json");
        let source = gen_file_path.as_path();
        let destination = dir.join("genesis.json");
        fs::copy(source, destination).unwrap();

        //blocks.db - fresh file
        File::create(dir.join("blocks.db")).unwrap();
    }
}
