mod login;
#[tokio::main]
async fn main() {
    // assuming your async function in `scraper.rs` is called `run`
    if let Err(e) = scraper::run().await {
        eprintln!("Failed to execute scraper: {}", e);
    }
}