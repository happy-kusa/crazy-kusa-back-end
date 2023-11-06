mod database;
mod models;
mod route;

use actix_web::{web::Data, App, HttpServer};
use database::mongodb::MongoRepo;
use route::idea_api;

const LISTEN_IP: &str = "127.0.0.1";
const LISTEN_PORT: u16 = 8080;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // 連線到 MongoDB
    let db = MongoRepo::init().await;
    let db_data = Data::new(db);

    // 啟動 HTTP Server
    println!("Start listening: {}:{}", LISTEN_IP, LISTEN_PORT);
    HttpServer::new(move || {
        App::new()
            .app_data(db_data.clone())
            .service(idea_api::create_idea)
            .service(idea_api::get_idea)
            .service(idea_api::update_idea)
            .service(idea_api::delete_idea)
            .service(idea_api::get_all_ideas)
    })
    .bind((LISTEN_IP, LISTEN_PORT))?
    .run()
    .await?;

    Ok(())
}
