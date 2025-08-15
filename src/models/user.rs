use sqlx::pool::Pool;
use sqlx::prelude::FromRow;
use sqlx::Postgres;
use sqlx::{query_as, query};

use crate::utils::random_id::generate_id_from_char_and_length;

pub struct User {
    pub ip_addr: String,
    pub vacuum: bool,
    pub level: usize,
    pub token: String,
}

#[derive(Debug, FromRow)]
pub struct UserRow {
    pub vacuum: bool,
    pub user_level: i32,
    pub token: String
}

#[derive(Debug, FromRow)]
pub struct UserRowIpAddr {
    pub vacuum: bool,
    pub user_level: i32,
    pub user_ip: String
}

const TOKEN_CHARSET: &str = "0123456789abcdef";
const TOKEN_LENGTH: usize = 32; // データベースではVARCHAR(32)

fn generate_token() -> String {
    generate_id_from_char_and_length(None, TOKEN_LENGTH, TOKEN_CHARSET)
}


impl User {
    pub async fn exist_check(ip_addr: &str) -> Result<bool, Box<dyn std::error::Error>> {
        
        let connect: Pool<Postgres> = super::connect_from_setting().await?;
        
        let users = query_as::<Postgres, UserRow>(include_str!("../../sql/user/user_get.sql"))
            .bind(ip_addr)
            .fetch_all(&connect).await?;
        Ok(!users.is_empty())
    }

    pub async fn register(ip_addr: &str) -> Result<Self, Box<dyn std::error::Error>>{
        let connect: Pool<Postgres> = super::connect_from_setting().await?;
        
        let token = generate_token();

        query::<Postgres>(include_str!("../../sql/user/user_register.sql"))
            .bind(ip_addr)
            .bind(&token)
            .execute(&connect).await?;

        Ok(Self {
            ip_addr: ip_addr.to_string(),
            vacuum: false,
            level: 1,
            token: token
        })

    }

    pub async fn from(ip_addr: &str) -> Result<Self, Box<dyn std::error::Error>> {
        
        let connect: Pool<Postgres> = super::connect_from_setting().await?;
        
        let users = query_as::<Postgres, UserRow>(include_str!("../../sql/user/user_get.sql"))
            .bind(ip_addr)
            .fetch_one(&connect).await?;


        Ok(Self {
            ip_addr: ip_addr.to_string(),
            vacuum: users.vacuum,
            level: users.user_level as usize,
            token: users.token,
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
        let connect: Pool<Postgres> = super::connect_from_setting().await?;
        
        query::<Postgres>(include_str!("../../sql/user/user_update.sql"))
            .bind(&self.vacuum)
            .bind(&(self.level as i32))
            .bind(&self.token)
            .bind(&self.ip_addr)
            .execute(&connect).await?;

        Ok(())
    }

    pub async fn migration_userip(&mut self, new_ipaddr: &str, new_token: &str) -> Result<(), Box<dyn std::error::Error>> {
        let connect: Pool<Postgres> = super::connect_from_setting().await?;
        
        query::<Postgres>(include_str!("../../sql/user/user_migration_delete_old_data.sql"))
            .bind(&self.token)
            .execute(&connect).await?;
    
        query::<Postgres>(include_str!("../../sql/user/user_migration.sql"))
            .bind(&new_ipaddr)
            .bind(new_token)
            .execute(&connect).await?;

        self.token = new_token.to_string();

        Ok(())
    }


    pub async fn from_token(token: &str) -> Result<Self, Box<dyn std::error::Error>> {
        
        let connect: Pool<Postgres> = super::connect_from_setting().await?;
        
        let users = query_as::<Postgres, UserRowIpAddr>(include_str!("../../sql/user/user_get_token.sql"))
            .bind(token)
            .fetch_one(&connect).await?;


        Ok(Self {
            ip_addr: users.user_ip.to_string(),
            vacuum: users.vacuum,
            level: users.user_level as usize,
            token: token.to_string(),
        })
    }

}

// この行以降は動作テスト

#[cfg(test)]
mod test {
    #[tokio::test]
    async fn test_token_generate() {
        let token = super::generate_token();
        println!("token生成テスト\t\t{}", token); // cargo test -- --nocapture
    }   

    #[tokio::test]
    async fn register_user() {
        let user = super::User::new("127.0.0.1").await.unwrap();
    }
}