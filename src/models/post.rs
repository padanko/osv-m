use crate::utils::{random_id, restriction};
use crate::SETTING;
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

#[derive(FromRow, Debug)]
pub struct PostRowId {
    pub post_name: String,
    pub user_id: String,
    pub date_time: NaiveDateTime,
    pub body: String,
    pub topic_id: String
}

// ここから実装


impl Post {
    pub fn new(
        name: &str,
        body: &str,
        user: &super::user::User,
    ) -> Self {
        Self {
            name: name.to_string(),
            body: body.to_string(),
            user_id: random_id::generate_user_view_id(&user.ip_addr),
            datetime: Local::now().naive_local()
        }
    }

    pub async fn commit(&self, bbs_id: &str, topic_id: &str) -> Result<(), Box<dyn std::error::Error>> {

        let connect: Pool<Postgres> = super::connect_from_setting().await?;

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

        let connect: Pool<Postgres> = super::connect_from_setting().await?;

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

    pub async fn headline(bbs_id: &str) -> Result<Vec<PostRowId>, Box<dyn std::error::Error>> {

        let connect: Pool<Postgres> = super::connect_from_setting().await?;

        let posts_ = query_as::<Postgres, PostRowId>(include_str!("../../sql/headline.sql"))
            .bind(bbs_id)
            .fetch_all(&connect).await?;

        Ok(posts_)

    }

    
}


// (規制: bool, 新しいハンドルネーム: String)
pub fn post_filter(body: &str, name: &str, bbs_id: &str) -> (bool, String) {
    if let Some(bbs_setting) = SETTING.bbs.get(bbs_id) {

        let default_name = &bbs_setting.default_name;

        
        let new_name = if name.is_empty() || bbs_setting.restriction_handlename {
            default_name.to_string()
        } else {
            name.to_string()
        };

        let is_body_lengthexceeds = body.chars().count() > bbs_setting.body_max_length;
        let is_name_lengthexceeds = name.chars().count() > bbs_setting.name_max_length;


        if is_body_lengthexceeds || is_name_lengthexceeds {
            return (true, new_name);
        }

        if restriction::body_check(body, bbs_setting) {
            return (true, new_name);
        }

        // ↓この機能は別のところで実装する
        //  if restriction::body_check(body, bbs_setting) || user.vacuum {
        //      user.vacuum = true;
        //      let _ = user.update().await;

            
        //  let period_vacuum = bbs_setting.vacuum_period_sec;

        //  tokio::spawn(async move {
        //      sleep(Duration::from_secs(period_vacuum)).await;
        //      user.vacuum = false;
        //      let _ = user.update().await;
        //  });

        //      return HttpResponse::Forbidden()
        //          .body(include_str!("../../default_html/error_unknown.html"));
        //  }

        // }
        (false, new_name)
    } else {
        (true, name.to_string())
    }
}