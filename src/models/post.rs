use crate::utils::random_id;

use sqlx::pool::Pool;
use sqlx::prelude::FromRow;
use sqlx::Postgres;
use sqlx::{query_as, query};
use chrono::{Local, NaiveDateTime};

#[derive(Debug)]
pub struct Post {
    pub name: String,
    pub user_id: String,
    pub datetime: NaiveDateTime,
    pub body: String
}

#[derive(FromRow, Debug)]
pub struct PostRow {
    pub post_name: String,
    pub user_id: String,
    pub date_time: NaiveDateTime,
    pub body: String
}


// ここから実装


impl Post {
    pub fn new(
        name: &str,
        body: &str,
        ip_addr: &str,
    ) -> Self {
        Self {
            name: name.to_string(),
            body: body.to_string(),
            user_id: random_id::generate_user_id(ip_addr),
            datetime: Local::now().naive_local()
        }
    }

    pub async fn commit(&self, bbs_id: &str, topic_id: &str) -> Result<(), Box<dyn std::error::Error>> {

        let connect: Pool<Postgres> = super::connect().await?;

        connect.begin().await?;

        query::<Postgres>(include_str!("../../sql/post_make.sql"))
            .bind(&self.name)
            .bind(&self.user_id)
            .bind(&self.datetime)
            .bind(&self.body)
            .bind(bbs_id)
            .bind(topic_id)
            .execute(&connect).await?;
        Ok(())
    }
    
    pub async fn from_vec(bbs_id: &str, topic_id: &str) -> Result<Vec<Post>, Box<dyn std::error::Error>> {

        let connect: Pool<Postgres> = super::connect().await?;

        let posts_ = query_as::<Postgres, PostRow>(include_str!("../../sql/posts_get.sql"))
            .bind(bbs_id)
            .bind(topic_id)
            .fetch_all(&connect).await?;

        let mut posts = Vec::new();

        for postrow in posts_ {
            posts.push(Post {
                name: postrow.post_name,
                user_id: postrow.user_id,
                datetime: postrow.date_time,
                body: postrow.body
            });
        }

        Ok(posts)

    }
}

