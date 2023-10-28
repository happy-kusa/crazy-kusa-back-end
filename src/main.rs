use server::server_entry;

// Import the rust file
mod server;

fn main() {

    if let Err(e) = server_entry() {
        eprintln!("Error: {}", e);
    }
}