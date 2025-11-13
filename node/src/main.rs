use database::fs::DataDir;

fn main() {
    println!("Migrating Data from Databases.!");
    DataDir::init(); //testing data directory configuration
}
