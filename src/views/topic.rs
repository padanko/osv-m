use crate::models::topic;

use actix_web::{web::{Data, Path}, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use crate::{ActixWebData, SETTING};

use tera::Context;

// GET /topic/◯◯

#[derive(Deserialize)]
pub struct BbsPath {
    pub bbs_id: String,
    pub topic_id: String
}

#[derive(Serialize)]
struct PostToString {
    name: String,
    datetime: String,
    id: String,
    body: String,
}

pub async fn endpoint(data: Data<ActixWebData>, bbspath: Path<BbsPath>) -> impl Responder {


    if let Some(bbs) = SETTING.bbs.get(&bbspath.bbs_id) {
        let mut ctx = Context::new();

        match topic::Topic::from(&bbspath.bbs_id, &bbspath.topic_id).await {
            Ok(topic) => {

                ctx.insert("title", &SETTING.title);
                ctx.insert("baseurl", &SETTING.base_url);
                ctx.insert("bbs_title", &bbs.title);
                ctx.insert("bbs_id", &bbspath.bbs_id);
                ctx.insert("topic_id", &bbspath.topic_id);
                ctx.insert("topic_title", &topic.topic_title);
                ctx.insert("password_enable", &(topic.topic_password != None));

                let mut posts_ctx = Vec::new();

                for post in topic.posts {
                    posts_ctx.push(PostToString {
                        name: post.name,
                        datetime: post.datetime.format(&SETTING.datetime_format).to_string(),
                        id: post.user_id,
                        body: post.body
                    })
                }

                ctx.insert("posts", &posts_ctx);

                match data.tera.render("topic.html", &ctx) {
                    Ok(html) => {
                        HttpResponse::Ok()
                            .body(html)
                    },
                    Err(_) => {
                        HttpResponse::InternalServerError()
                            .body(include_str!("../../default_html/error_render.html"))
                    }
                }
            },
            Err(_) => {
                HttpResponse::InternalServerError()
                   .body(include_str!("../../default_html/error_not_found_topic.html"))
            }       
        }
    } else {
        HttpResponse::InternalServerError()
           .body(include_str!("../../default_html/error_not_found_board.html"))
    
    }

}