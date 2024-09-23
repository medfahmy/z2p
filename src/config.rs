#[derive(serde::Deserialize)]
pub struct Config {
    pub host: String,
    pub port: u16,
    pub db_url: String,
}

impl Config {
    pub fn get() -> Self {
        let config = config::Config::builder()
            .add_source(config::File::new("config.toml", config::FileFormat::Toml))
            .build()
            .unwrap();

        config.try_deserialize::<Self>().unwrap()
    }
}
