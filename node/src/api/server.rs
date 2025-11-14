use crate::api::{balances, tx};
use actix_web::{App, HttpServer};

pub struct ApiServer {}

impl ApiServer {
    pub async fn run() -> std::io::Result<()> {
        let server =
            HttpServer::new(|| App::new().service(tx::router()).service(balances::router()))
                .bind(("127.0.0.1", 8080))?;

        println!("🚀 Server running at PORT 8080");
        server.run().await?;

        println!("Una reach here");
        Ok(())
    }
}
