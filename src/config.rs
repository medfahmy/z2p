use once_cell::sync::Lazy;
use secrecy::{SecretString, ExposeSecret};

pub static CONFIG: Lazy<Config> = Lazy::new(|| Config::init());

#[derive(serde::Deserialize)]
pub struct Config {
    pub server: Server,
    pub db: DB,
}

impl Config {
    pub fn init() -> Self {
        let config = config::Config::builder()
            .add_source(config::File::with_name(&format!("config.toml")))
            .build()
            .unwrap();

        config.try_deserialize::<Self>().unwrap()
    }

    pub fn addr(&self) -> String {
        format!("{}:{}", self.server.host, self.server.port)
    }

    pub fn pg_url(&self) -> SecretString {
        SecretString::from(format!(
            "postgres://{}:{}@{}:{}",
            self.db.user, self.db.pass.expose_secret(), self.db.host, self.db.port,
        ))
    }

    pub fn db_url(&self) -> SecretString {
        SecretString::from(format!(
            "postgres://{}:{}@{}:{}/{}",
            self.db.user, self.db.pass.expose_secret(), self.db.host, self.db.port, self.db.name,
        ))
    }
}

#[derive(serde::Deserialize)]
pub struct Server {
    pub host: String,
    pub port: u16,
}

#[derive(serde::Deserialize)]
pub struct DB {
    pub host: String,
    pub port: u16,
    pub user: String,
    pub pass: SecretString,
    pub name: String,
}
