use sqlx::pool::Pool;
use sqlx::prelude::FromRow;
use sqlx::Postgres;
use sqlx::{query_as, query};

pub struct User {
    pub ip_addr: String,
    pub vacuum: bool,
    pub level: usize
}

#[derive(Debug, FromRow)]
pub struct UserRow {
    pub vacuum: bool,
    pub user_level: i32
}

impl User {
    pub async fn exist_check(ip_addr: &str) -> Result<bool, Box<dyn std::error::Error>> {
        
        let connect: Pool<Postgres> = super::connect().await?;
        
        let users = query_as::<Postgres, UserRow>(include_str!("../../sql/user/user_get.sql"))
            .bind(ip_addr)
            .fetch_all(&connect).await?;
        Ok(!users.is_empty())
    }

    pub async fn register(ip_addr: &str) -> Result<Self, Box<dyn std::error::Error>>{
        let connect: Pool<Postgres> = super::connect().await?;
        
        query::<Postgres>(include_str!("../../sql/user/user_register.sql"))
            .bind(ip_addr)
            .execute(&connect).await?;

        Ok(Self {
            ip_addr: ip_addr.to_string(),
            vacuum: false,
            level: 1
        })

    }

    pub async fn from(ip_addr: &str) -> Result<Self, Box<dyn std::error::Error>> {
        
        let connect: Pool<Postgres> = super::connect().await?;
        
        let users = query_as::<Postgres, UserRow>(include_str!("../../sql/user/user_get.sql"))
            .bind(ip_addr)
            .fetch_one(&connect).await?;


        Ok(Self {
            ip_addr: ip_addr.to_string(),
            vacuum: users.vacuum,
            level: users.user_level as usize
        })
    }

    pub async fn new(ip_addr: &str) -> Result<Self, Box<dyn std::error::Error>> {
        if !Self::exist_check(ip_addr).await? {
            Ok(Self::register(ip_addr).await?)
        } else {
            Ok(Self::from(ip_addr).await?)
        }
    }

    pub async fn update(&self) -> Result<(), Box<dyn std::error::Error>> {
        let connect: Pool<Postgres> = super::connect().await?;
        
        query::<Postgres>(include_str!("../../sql/user/user_update.sql"))
            .bind(&self.vacuum)
            .bind(&(self.level as i32))
            .bind(&self.ip_addr)
            .execute(&connect).await?;

        Ok(())
    }

}