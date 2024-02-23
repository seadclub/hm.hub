pub mod db;
pub mod telegram;
pub mod utils;

use crate::errors::Result;
use crate::models::State;
use db::create_db;
use dotenv::dotenv;
use std::sync::Arc;
use telegram::handler::schema;
use teloxide::dispatching::dialogue::InMemStorage;
use teloxide::prelude::*;

mod errors;
mod models;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();
    log::info!("Starting...");
    create_db().expect("Failed to create database at main");

    let bot = Bot::new(dotenv::var("TELOXIDE_TOKEN")?);
    let state = Arc::new(State::Start);

    Dispatcher::builder(bot, schema())
        .dependencies(dptree::deps![
            InMemStorage::<State>::new(),
            Arc::clone(&state)
        ])
        .enable_ctrlc_handler()
        .build()
        .dispatch()
        .await;
    Ok(())
}
