use actix_web::{Responder, HttpResponse};
use actix_web::web::Path;
use serde::Serialize;
use crate::models::post::Post;
use crate::models::topic::Topic;
use crate::SETTING;


#[derive(Serialize)]
struct ResponsePost {
    datetime: String,
    body: String,
    topicid: String,
    title: String,
    count: usize
}

#[derive(Serialize)]
struct ResponseHeadline {
    data: Vec<ResponsePost>
}

#[derive(Serialize)]
struct ResponseError {
    error_message: String
}

pub async fn endpoint(bbspath: Path<super::BbsPath>) -> impl Responder {

    let bbs_id = &bbspath.bbs_id;

    match Post::headline(bbs_id).await {
        Ok(posts) => {

            let mut headline: Vec<ResponsePost> = Vec::new();

            for post in posts {

                let topic = Topic::from(bbs_id, &post.topic_id).await;
                if let Ok(topic) = topic{
                    let count = topic.posts.len();
                    let title = topic.topic_title;

                    let post_ = ResponsePost {
                        datetime: post.date_time.format(&SETTING.datetime_format).to_string(),
                        body: post.body,
                        topicid: post.topic_id,
                        title: title,
                        count: count
                    };

                    headline.push(post_);
                }
            }
            let headline = ResponseHeadline {
                data: headline
            };

            HttpResponse::Ok()
                .content_type("application/json")
                .body(serde_json::to_string(&headline).unwrap())
        },
        Err(e) => {
            eprintln!("エラー\t\t{}", e);    
            HttpResponse::InternalServerError()
                .content_type("application/json")
                .body(serde_json::to_string(&ResponseError { error_message: e.to_string() }).unwrap())
        }
    }

}