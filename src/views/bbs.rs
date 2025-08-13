use crate::models::topic;

use actix_web::{web::{Data, Path}, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use crate::{ActixWebData, SETTING};

use tera::Context;

// GET /bbstable/

#[derive(Deserialize)]
pub struct BbsPath {
    pub bbs_id: String
}

#[derive(Serialize)]
struct TopicToString {
    title: String,
    topicid: String,
    count: usize
}

pub async fn endpoint(data: Data<ActixWebData>, bbspath: Path<BbsPath>) -> impl Responder {


    if let Some(bbs) = SETTING.bbs.get(&bbspath.bbs_id) {
        let mut ctx = Context::new();

        ctx.insert("title", &SETTING.title);

        ctx.insert("bbs_id", &bbspath.bbs_id);
        ctx.insert("bbs_title", &bbs.title);
        ctx.insert("bbs_description", &bbs.description_html);
        
        ctx.insert("baseurl", &SETTING.base_url);
        ctx.insert("bannerurl", &bbs.banner);

        match topic::Topic::from_vec(&bbspath.bbs_id).await {
            Ok(topics) => {
                let mut topic_ctx: Vec<TopicToString> = Vec::new();

                for topic in topics {
                    topic_ctx.push(TopicToString {
                        title: topic.topic_title,
                        topicid: topic.topic_id,
                        count: topic.posts.len()
                    });
                }

                ctx.insert("topics", &topic_ctx);

                match data.tera.render("bbspage.html", &ctx) {
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
            Err(e) => {
                eprintln!("{}", e);
                HttpResponse::InternalServerError()
                    .body(include_str!("../../default_html/error_cannot_load_topics.html"))
            }
        }
    } else {
        HttpResponse::InternalServerError()
           .body(include_str!("../../default_html/error_not_found_board.html"))
    
    }

}