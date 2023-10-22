use reqwest::Client;
use serde::Deserialize;
use std::error::Error;
use std::fs;
use chrono::{Local, Datelike, Timelike};

// actor web
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

// Import the rust file
mod file_handling;  
mod mongo_connect;


#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[derive(Deserialize)]
struct Config {
    url: String,
}

#[actix_web::main]
async fn server() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(echo)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {

    // 啟動非同步
    tokio::spawn(async {
        // 啟動 Web 伺服器
        let server_result = server();
        if let Err(err) = server_result {
            eprintln!("Server error: {:?}", err);
        }
    });

    // Connect to MongoDB
    let _client = mongo_connect::connect_to_mongodb().await?;
    println!("Connected to MongoDB successfully!");

    // 讀取配置
    let config_content = fs::read_to_string("config.yaml")?;
    let config: Config = serde_yaml::from_str(&config_content)?;

    // 取得現在時間
    let current_time = Local::now();
    let formatted_time = format!(
        "data_{}-{}-{}_{}-{}-{}.json",
        current_time.year(),
        current_time.month(),
        current_time.day(),
        current_time.hour(),
        current_time.minute(),
        current_time.second()
    );

    let file_path = format!("./assets/raw/{}", formatted_time);

    // 發送
    let client = Client::new();
    let response = client.get(&config.url).send().await?;

    // 檢查response回傳值
    if response.status().is_success() {
        // 解析response
        let body = response.text().await?;

        file_handling::save_to_file(&file_path, &body)?;
        println!("Response body saved to {}", &file_path);
    } else {
        println!("Request failed with status code: {}", response.status());
    }

    // 阻塞主執行緒
    // tokio::signal::ctrl_c().await.expect("Error waiting for Ctrl-C");

    Ok(())
}
