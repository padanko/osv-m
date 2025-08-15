pub mod topic;
pub mod post;
pub mod user;

use crate::SETTING;
use sqlx::{Pool, Postgres};

pub async fn connect_from_setting() -> Result<Pool<Postgres>, Box<dyn std::error::Error>> {
    Ok(connect(&SETTING.postgresql_username,
        &SETTING.postgresql_password,
        &SETTING.postgresql_addr,
        SETTING.postgresql_port,
        &SETTING.postgresql_database).await?)
}

pub async fn connect(username: &str, password: &str, addr: &str, port: u16, database: &str) -> Result<Pool<Postgres>, Box<dyn std::error::Error>> {
    Ok(Pool::connect(&format!(
        "postgres://{}:{}@{}:{}/{}",
        username, password, addr, port, database
    )).await?)
}