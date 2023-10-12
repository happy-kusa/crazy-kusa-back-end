use reqwest::Client;
use serde::Deserialize;
use std::error::Error;
use std::fs;
use std::io::Write;
use chrono::{Local, Datelike, Timelike};

#[derive(Deserialize)]
struct Config {
    url: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
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

        // 生成文件與組合路徑資訊
        save_to_file(&file_path, &body)?;
        println!("Response body saved to {}", &file_path);
    } else {
        println!("Request failed with status code: {}", response.status());
    }

    Ok(())
}

fn save_to_file(file_path: &str, content: &str) -> Result<(), Box<dyn Error>> {
    let mut file = std::fs::File::create(file_path)?;
    file.write_all(content.as_bytes())?;
    Ok(())
}
