use teloxide::Bot;
use teloxide::prelude::Message;
use teloxide::types::Me;
use crate::datasource::config::{create_pool, Error};

pub async fn run() -> Result<(), Box<Error>> {
    pretty_env_logger::init();
    log::info!("starting bot");

    let bot = Bot::from_env();
    let pool = create_pool(true).await?;
    Ok(())
}

async fn message_handler(bot: &Bot, message: Message, me: Me) {
    message.via_bot.unwrap().
}