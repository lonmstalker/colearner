use crate::datasource::config::create_pool;

mod app;
mod datasource;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let pool = create_pool(true);
}
