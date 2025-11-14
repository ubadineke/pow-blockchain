// use actix_web::{App, HttpServer};
// use database::fs::DataDir;
use node::api::server::ApiServer;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    println!("Migrating Data from Databases.!");
    // DataDir::init(); //testing data directory configuration
    ApiServer::run().await?;
    Ok(())
}
