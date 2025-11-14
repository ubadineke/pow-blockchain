use node::api::server::ApiServer;

pub async fn run_node() -> std::io::Result<()> {
    println!("Launch node and its HTTP API");

    ApiServer::run().await?;
    Ok(())
}
