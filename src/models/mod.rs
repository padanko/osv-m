pub mod topic;
pub mod post;
pub mod user;

use crate::SETTING;
use sqlx::{Pool, Postgres};

pub async fn connect() -> Result<Pool<Postgres>, Box<dyn std::error::Error>> {
    Ok(Pool::connect(&format!(
        "postgres://{}:{}@{}:{}/{}",
        &SETTING.postgresql_username,
        &SETTING.postgresql_password,
        &SETTING.postgresql_addr,
        &SETTING.postgresql_port,
        &SETTING.postgresql_database,
    )).await?)
}