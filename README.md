# Crazy Kusa Back End

這是一個某位想不開工程師的自虐之作

### 資料庫

mongodb+srv://chris:chris1234@chris-db.ec2i5ii.mongodb.net/?retryWrites=true&w=majority

### 路由

| 功能   | 方法   | 路徑                               | 作用   | 描述 |
| ------ | ------ | ---------------------------------- | ------ | ---- |
| 讀全部 | GET    | http://127.0.0.1:8080/blogs        | 資料庫 |      |
| 建立   | POET   | http://127.0.0.1:8080/blog         | 資料庫 |      |
| 讀取   | GET    | http://127.0.0.1:8080/blog/`<OID>` | 資料庫 |      |
| 更新   | PUT    | http://127.0.0.1:8080/blog/`<OID>` | 資料庫 |      |
| 刪除   | DELETE | http://127.0.0.1:8080/blog/`<OID>` | 資料庫 |      |

### 執行

-   編譯 + 執行: `cargo run`
-   編譯: `cargo build`
-   編譯(發布用): `cargo build --release`
-   檢查是否能編譯: `cargo check`

### 命名規則

-   全域變數 環境變數 靜態 常數 => 全大寫用\_分開
-   變數 => 全小寫用\_分開
-   類別 特徵 結構 列舉 => 大寫駝峰
-   函數 方法 模組 檔案 => 全小寫用\_分開
-   目錄 => 全小寫用\-分開

### 參考與教學

https://course.rs/practice/naming.html

https://dev.to/hackmamba/build-a-rest-api-with-rust-and-mongodb-actix-web-version-ei1
