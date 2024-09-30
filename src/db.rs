use crate::Config;
use sqlx::{Connection, Executor, PgConnection, PgPool};
use secrecy::ExposeSecret;

pub async fn init_db(config: &Config, test: bool) -> PgPool {
    if !test {
        let db_url = &config.db_url().expose_secret().to_string();
        tracing::info!("connected to db on '{}'", db_url);
        return PgPool::connect(db_url).await.unwrap();
    }

    let pg_url = &config.pg_url().expose_secret().to_string();
    let db_name = uuid::Uuid::new_v4().to_string();

    PgConnection::connect(pg_url)
        .await
        .unwrap()
        .execute(format!("create database \"{}\";", &db_name).as_str())
        .await
        .unwrap();

    let pool = PgPool::connect(&format!("{}/{}", pg_url, db_name))
        .await
        .unwrap();

    sqlx::migrate!("./migrations").run(&pool).await.unwrap();

    pool
}
