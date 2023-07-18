use std::io::ErrorKind;
use deadpool_postgres::{ManagerConfig, Pool, Runtime};
use deadpool_postgres::RecyclingMethod::Fast;
use serde::Deserialize;
use tokio_postgres::NoTls;
use crate::datasource::migrations::run_migrations;

pub(crate) type Error = std::io::Error;

pub async fn create_pool(with_migrations: bool) -> Result<Pool, Error> {
    let props = envy::from_env::<PostgresProperties>()
        .map_err(|e| Error::new(ErrorKind::InvalidInput, e.to_string()))?;

    if with_migrations {
        run_migrations(&props);
    }

    let pool = props
        .to_config()
        .create_pool(Some(Runtime::Tokio1), NoTls)
        .map_err(|e| Error::new(ErrorKind::InvalidInput, e.to_string()));

    if let Ok(ref pl) = pool {
        validate(pl);
    }

    log::info!("pool created");
    pool
}

async fn validate(pool: &Pool) -> Result<(), Error> {
    pool.get()
        .await
        .map_err(|e| Error::new(ErrorKind::NotConnected, e.to_string()))
        ?.query("SELECT 1", &[])
        .await
        .map_err(|e| Error::new(ErrorKind::NotConnected, e.to_string()))
        ?.get(0)
        .map(|res| {
            let rs: i32 = res.get(0);
            assert_eq!(1, rs, "incorrect response of db");
        })
        .ok_or(Error::new(ErrorKind::InvalidInput, "can't connect to db"))
}

#[derive(Deserialize, Debug)]
pub struct PostgresProperties {
    #[serde(default = "postgres")]
    pub db_name: String,

    #[serde(default = "host")]
    pub db_host: String,

    #[serde(default = "port")]
    pub db_port: u16,

    #[serde(default = "postgres")]
    pub db_user: String,

    #[serde(default = "postgres")]
    pub db_password: String,
}

impl PostgresProperties {
    fn to_config(self) -> deadpool_postgres::Config {
        let mut config = deadpool_postgres::Config::new();

        config.user = Option::from(self.db_user);
        config.host = Option::from(self.db_host);
        config.port = Option::from(self.db_port);
        config.dbname = Option::from(self.db_name);
        config.password = Option::from(self.db_password);
        config.manager = Option::from(ManagerConfig { recycling_method: Fast });

        config
    }
}

fn postgres() -> String {
    String::from("postgres")
}

fn host() -> String {
    String::from("localhost")
}

fn port() -> u16 {
    5432
}