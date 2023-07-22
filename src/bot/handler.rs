use std::sync::Arc;
use teloxide::Bot;
use teloxide::prelude::Message;
use crate::app::app::AppState;
use crate::datasource::config::{create_pool, Error};

pub async fn run() -> Result<(), Box<Error>> {
    pretty_env_logger::init();
    log::info!("starting bot");

    let bot = Bot::from_env();
    let pool = create_pool(true).await?;

    let state = Arc::new(AppState { pool });
    Ok(())
}

async fn message_handler(bot: &Bot, message: Message, state: AppState) {
    message.via_bot.unwrap().
}