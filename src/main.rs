pub mod db;

use db::create_db;

#[tokio::main]
async fn main() {
    log::info!("Starting dialogue bot...");
    create_db();
    pretty_env_logger::init();
}
