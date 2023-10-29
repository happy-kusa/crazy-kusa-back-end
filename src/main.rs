use server::server_entry;

mod server;

fn main() {
    // 呼叫 server_entry() 函數，並 Result 錯誤狀況
    if let Err(err) = server_entry() {
        eprintln!("Error: {}", err);
    }
}