use deadpool_postgres::ManagerConfig;
use deadpool_postgres::RecyclingMethod::Fast;

pub(crate) type Error = std::io::Error;

async fn validate() {
    
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