use crate::utils::random_id;
use super::post::Post;

use sqlx::pool::Pool;
use sqlx::prelude::FromRow;
use sqlx::Postgres;
use sqlx::{query_as, query};


#[derive(FromRow, Debug)]
pub struct TopicRow {
    pub topic_id: String,
    pub topic_title: String,
    pub topic_password: Option<String>,
    pub topic_default_name: Option<String>,
}

#[derive(Debug)]

pub struct Topic {
    pub topic_id: String,
    pub topic_title: String,
    pub topic_password: Option<String>,
    pub topic_default_name: Option<String>,
    pub bbs_id: String,
    pub posts: Vec<super::post::Post>, 
}


// ここから実装

impl Topic {
    pub fn new(
        title: &str,
        password: Option<String>,
        default_name: Option<String>,
        bbs_id: &str,
    ) -> Self {
        Self {
            topic_id: random_id::generate_topic_id(),
            topic_title: title.to_string(),
            topic_password: password,
            topic_default_name: default_name,
            bbs_id: bbs_id.to_string(),
            posts: Vec::new()
        }
    }

    pub async fn commit(&self) -> Result<(), Box<dyn std::error::Error>> {

        let connect: Pool<Postgres> = super::connect_from_setting().await?;

        connect.begin().await?;

        query::<Postgres>(include_str!("../../sql/topic_make.sql"))
            .bind(&self.topic_title)
            .bind(&self.topic_id)
            .bind(&self.topic_password)
            .bind(&self.topic_default_name)
            .bind(&self.bbs_id)
            .execute(&connect).await?;

            // topic_title | topic_id | topic_password | topic_default_name | bbs_id

        
        Ok(())
    }

    pub async fn post(&self, post: super::post::Post) -> Result<(), Box<dyn std::error::Error>> {
        post.commit(&self.bbs_id, &self.topic_id).await?;
        Ok(())
    }

    pub async fn from(bbs_id: &str, topic_id: &str) -> Result<Self, Box<dyn std::error::Error>> {

        let connect: Pool<Postgres> = super::connect_from_setting().await?;

        let topic = query_as::<Postgres, TopicRow>(include_str!("../../sql/topic_get.sql"))
            .bind(bbs_id)
            .bind(topic_id)
            .fetch_one(&connect).await?;

        let topic = Self {
            topic_id: topic.topic_id,
            topic_title: topic.topic_title,
            topic_password: topic.topic_password,
            topic_default_name: topic.topic_default_name,
            bbs_id: bbs_id.to_string(),
            posts: Post::from_vec(bbs_id, topic_id).await?
        };

        Ok(topic)

    }

    pub async fn from_vec(bbs_id: &str) -> Result<Vec<Self>, Box<dyn std::error::Error>> {

        let connect: Pool<Postgres> = super::connect_from_setting().await?;

        let topics_ = query_as::<Postgres, TopicRow>(include_str!("../../sql/topics_get.sql"))
            .bind(bbs_id)
            .fetch_all(&connect).await?;

        let mut topics: Vec<Topic> = Vec::new();

        for topic in &topics_ {


            let topic_ = Self {
                topic_id: topic.topic_id.to_string(),
                topic_title: topic.topic_title.to_string(),
                topic_password: topic.topic_password.clone(),
                topic_default_name: topic.topic_default_name.clone(),
                bbs_id: bbs_id.to_string(),
                posts: Post::from_vec(bbs_id, &topic.topic_id.clone()).await?
            };

            topics.push(topic_);

        }

        Ok(topics)

    }
}
