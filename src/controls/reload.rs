use actix_web::{Responder, HttpResponse};
use actix_web::web::Path;
use tokio::time::{sleep, Duration};
use serde::Serialize;
use crate::models::post::Post;
use crate::{POLL_INSTANCE, SETTING};

const DELAY_MS: u64 = 500;

#[derive(Serialize)]
struct ResponseLatestPost {
    name: String,
    datetime: String,
    id: String,
    body: String
}

#[derive(Serialize)]
struct ResponseError {
    error_message: String
}

pub async fn endpoint(bbspath: Path<super::BbsTopicPath>) -> impl Responder {

    let bbs_id = &bbspath.bbs_id;
    let topic_id = &bbspath.topic_id;

    let mut old = {
        let poll_instance = POLL_INSTANCE.lock().unwrap();

        poll_instance.randvalue.clone()
    };

    loop {

        sleep(Duration::from_millis(DELAY_MS)).await;

        let poll_instance = POLL_INSTANCE.lock().unwrap().clone();

        let bbs_id_ = &poll_instance.bbs_id;
        let topic_id_ = &poll_instance.topic_id;

        let new = poll_instance.randvalue;

        if old != new && bbs_id == bbs_id_ && topic_id == topic_id_{

            match Post::from_vec(bbs_id, topic_id).await {
                Ok(posts) => {

                    // SQL使うなりもっと良い方法があるけど今はこれで
                    if let Some(latest) = posts.get(posts.len()-1) {

                        let response = ResponseLatestPost {
                            name: latest.name.clone(),
                            datetime: latest.datetime.format(&SETTING.datetime_format).to_string(),
                            id: latest.user_id.clone(),
                            body: latest.body.clone()
                        };

                        return HttpResponse::Ok()
                            .content_type("application/json")
                            .body(serde_json::to_string(&response).unwrap());

                    } else {
                        return HttpResponse::InternalServerError()
                            .content_type("application/json")
                            .body(serde_json::to_string(&ResponseError {
                                error_message: String::from("投稿がない")
                            }).unwrap());
                    }

                },
                Err(e) => {

                    return HttpResponse::InternalServerError()
                        .content_type("application/json")
                        .body(serde_json::to_string(&ResponseError {
                            error_message: e.to_string()
                        }).unwrap());
                }
            }
        }

        old = new.clone();

    }

}