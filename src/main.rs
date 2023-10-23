// 引入 Actix Web
use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use std::sync::Mutex;

// This struct represents state
struct AppState {
    app_name: String,
}

struct AppStateWithCounter {
    counter: Mutex<i32>, // <- Mutex is necessary to mutate safely across threads
}

#[get("/")]
async fn index_1(data: web::Data<AppState>) -> String {
    let app_name = &data.app_name; // <- get app_name
    format!("Hello {app_name}!") // <- response with app_name
}

async fn index_2(data: web::Data<AppStateWithCounter>) -> String {
    let mut counter = data.counter.lock().unwrap(); // <- get counter's MutexGuard
    *counter += 1; // <- access counter inside MutexGuard

    format!("Request number: {counter}") // <- response with count
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        // App::new().service(
        //     web::scope("/app")
        //         .route("/index.html", web::get().to(index)),
        // )
        App::new().app_data(web::Data::new(AppState {
                app_name: String::from("Actix Web"),
            }))
            .service(index_1)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}