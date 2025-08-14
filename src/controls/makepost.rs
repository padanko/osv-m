use actix_web::{HttpRequest, Responder, HttpResponse};
use actix_web::web::{Form, Path};
use serde::Deserialize;
use crate::models::post::Post;
use crate::models::topic::Topic;
use crate::utils::random_id::random_integer;
use crate::utils::restriction;
use crate::{utils, PollState, POLL_INSTANCE, SETTING};


#[derive(Deserialize)]
pub struct MakeTopicJson {
    name: String,
    body: String,
    password: Option<String>,
}


// /◯◯/make/post formタグでやる
pub async fn endpoint(req: HttpRequest, bbspath: Path<super::BbsTopicPath>, data: Form<MakeTopicJson>) -> impl Responder {

    let mut name = &utils::html::html_escape(&data.name);
    let body = &utils::html::html_escape(&data.body);

    let body = if SETTING.enable_command {
        &crate::commands::apply_all(&body)
    } else {
        body
    };
    
    let ip_addr = utils::get_ip::get_ipaddr_from_header(&req).unwrap_or(String::from("???"));

    let bbs_ = &SETTING.bbs;

    if let Some(bbs_setting) = bbs_.get(&bbspath.bbs_id) {
        let default_name = &bbs_setting.default_name;
        if name.is_empty() || bbs_setting.restriction_handlename{
            name = default_name;
        }

        let has_body_lengthexceeds = body.chars().count() > bbs_setting.body_max_length;
        let has_name_lengthexceeds = name.chars().count() > bbs_setting.name_max_length;
        
        if has_body_lengthexceeds || has_name_lengthexceeds {
            return HttpResponse::Forbidden()
                .body(include_str!("../../default_html/error_has_length_exceeds.html"));
        }

        if restriction::body_check(body, bbs_setting) {
            return HttpResponse::Forbidden()
                .body(include_str!("../../default_html/error_unknown.html"));
        }

    }

    
    if !body.is_empty() {


        let topic = Topic::from(&bbspath.bbs_id, &bbspath.topic_id).await;

        match topic {

            Ok(topic) => {

                let password = topic.topic_password.clone();

                let mut password_is_valid = true;

                if let (Some(password_), Some(password_userinput)) = (password, data.password.clone()) {
                    password_is_valid = password_ == password_userinput;
                }

                if password_is_valid {

                    let post = Post::new(name, body, &ip_addr);
                    
                    match topic.post(post).await {
                        Ok(()) => {
                            let mut poll_instance = POLL_INSTANCE.lock().unwrap();

                            *poll_instance = PollState {
                                topic_id: bbspath.topic_id.clone(),
                                randvalue: random_integer(64),
                                bbs_id: bbspath.bbs_id.clone()
                            };

                            return HttpResponse::Ok()
                                .body("OK");
                        },
                        Err(_) => {
                            HttpResponse::InternalServerError()
                                .body(include_str!("../../default_html/error_unknown.html"))

                        }
                    }
                } else {
                    HttpResponse::Forbidden()
                        .body("PASSWORD IS INVALID")
                }
            },
            Err(_) => {
                HttpResponse::InternalServerError()
                    .body(include_str!("../../default_html/error_unknown.html"))

            } 
        }
    } else {
        HttpResponse::Forbidden()
            .body(include_str!("../../default_html/error_title_or_body_is_empty.html"))
    }
}