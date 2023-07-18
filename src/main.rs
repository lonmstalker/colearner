use teloxide::Bot;
use crate::bot::handler;
use crate::datasource::config::create_pool;

mod app;
mod bot;
mod service;
mod datasource;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    return handler::run();
}
