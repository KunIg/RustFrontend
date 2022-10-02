#[macro_use]
extern crate diesel;

use actix_web::{App, HttpServer, web::Data};
use anyhow::Result;
use diesel::r2d2::ConnectionManager;
use hackathon3::{routes::{query1, ping_pong}, types::{AppState, Pool}};

#[actix_web::main]
async fn main() -> Result<()> {
    dotenv::dotenv().ok();
    env_logger::init();
    let app_name = String::from("hackathon3");

    let database_url = std::env::var("DATABASE_URL").expect("NOT FOUND");
    let pool = Pool::builder()
        .build(ConnectionManager::new(database_url))
        .unwrap();

    HttpServer::new(move || {
        let state = AppState {
            app_name: app_name.clone(),
            pool: pool.clone(),
        };
        App::new()
            .app_data(Data::new(state))
            .service(ping_pong)
            .service(query1)
    })
    .bind(("127.0.0.1", 8081))?
    .run()
    .await?;

    Ok(())
}
