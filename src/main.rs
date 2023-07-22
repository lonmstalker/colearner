use crate::bot::handler;

mod app;
mod bot;
mod service;
mod datasource;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    handler::run()
}
