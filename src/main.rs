pub mod telegram;
pub mod db;
use crate::errors::Result;
use crate::commands::schema;
use crate::models::State;
use dotenv::dotenv;
use std::sync::Arc;
use db::create_db;
use teloxide::dispatching::dialogue::InMemStorage;
use teloxide::prelude::*;

mod models;
mod commands;
mod errors;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();
    log::info!("Starting dialogue bot...");
    create_db();

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
