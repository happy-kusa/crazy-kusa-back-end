use actix_web::{web, App, HttpResponse, HttpServer, Responder, Result};

mod route;
mod mongo_connect;

async fn root_handler() -> impl Responder {
    HttpResponse::Ok().body("Hey there is root!")
}

#[actix_web::main]
pub async fn server_entry() -> Result<(), std::io::Error> {

    // Connect to MongoDB
    println!("Waiting connect to MongoDB...");
    let _client = mongo_connect::connect_to_mongodb().await;
    println!("Connected to MongoDB successfully! _client = {:?}", _client);

    // Start HTTP Server
    println!("Start listening: 127.0.0.1:8080");
    HttpServer::new(|| {
        App::new()
            .service(web::scope("/chris").configure(route::chris_cfg_fn))
            .service(web::scope("/mongo").configure(route::mongo_cfg_fn))
            .route(
                "/",
                web::get().to(root_handler),
            )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await?;

    Ok(())
}

