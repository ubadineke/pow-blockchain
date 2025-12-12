// use actix_web::{App, HttpServer};
// use database::fs::DataDir;
use node::{api::server::ApiServer, migrate::migrate_db};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    println!("Migrating Data from Databases!");
    // DataDir::init(); //testing data directory configuration
    // ApiServer::run().await?;
    migrate_db().unwrap(); //convert to a cli command.(remove existing blocks.db, create new one and write)
    Ok(())
}
