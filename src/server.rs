use actix_web::{web, App, HttpResponse, HttpServer, Responder, Result};

mod route;
mod mongo;

const LISTEN_IP: &str = "127.0.0.1";
const LISTEN_PORT: u16 = 8080;

// Handler 根目錄
async fn root_handler() -> impl Responder {
    HttpResponse::Ok().body("Hey there is root!")
}

#[actix_web::main]
pub async fn server_entry() -> Result<(), std::io::Error> {

    // 連線到 MongoDB
    println!("Waiting connect to MongoDB...");
    let _client = mongo::connect_to_mongodb().await;
    // println!("Connected to MongoDB successfully! _client = {:?}", _client);

    // 啟動 HTTP Server
    println!("Start listening: {}:{}", LISTEN_IP, LISTEN_PORT);
    HttpServer::new(|| {
        App::new()
            .service(web::scope("/chris").configure(route::chris_cfg_fn))
            .service(web::scope("/mongo").configure(route::mongo_cfg_fn))
            .route(
                "/",
                web::get().to(root_handler),
            )
    })
    .bind((LISTEN_IP, LISTEN_PORT))?
    .run()
    .await?;

    Ok(())
}

