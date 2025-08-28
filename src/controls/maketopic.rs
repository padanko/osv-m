use actix_web::{HttpRequest, Responder, HttpResponse};
use actix_web::web::{Form, Path};
use serde::Deserialize;

// use tokio::time::{Duration, sleep};

use crate::models::user::User;
use crate::models::post::{self, Post};
use crate::models::topic::Topic;
// use crate::utils::restriction;
use crate::{utils, SETTING};


#[derive(Deserialize)]
pub struct MakeTopicForm {
    title: String,
    name: String,
    body: String,
    password: String,
}

// /◯◯/make/topic formタグでやる
pub async fn endpoint(req: HttpRequest, bbspath: Path<super::BbsPath>, data: Form<MakeTopicForm>) -> impl Responder {

    let title = &data.title;
    let name = &utils::html::html_escape(&data.name);
    let body = &utils::html::html_escape(&data.body);
    let password = if (&data).password.is_empty() { None } else { Some(data.password.clone()) };

    let ip_addr = utils::get_ip::get_ipaddr_from_header(&req).unwrap_or(String::from("???"));

    let user = if let Ok(user) = User::new(&ip_addr).await {
        user
    } else {
        return HttpResponse::InternalServerError()
            .body(include_str!("../../default_html/error_user_get.html"))
    };

    let body = if SETTING.enable_command {
        &crate::commands::apply_all(&body, &user, &bbspath.bbs_id)
    } else {
        body
    };


    // let bbs_ = &SETTING.bbs;

    // if let Some(bbs_setting) = bbs_.get(&bbspath.bbs_id) {
    //     let default_name = &bbs_setting.default_name;
    //     if name.is_empty() || bbs_setting.restriction_handlename{
    //         name = default_name;
    //     }

    //     let has_title_lengthexceeds = title.chars().count() > bbs_setting.title_max_length;
    //     let has_body_lengthexceeds = body.chars().count() > bbs_setting.body_max_length;
    //     let has_name_lengthexceeds = name.chars().count() > bbs_setting.name_max_length;
        
    //     if has_title_lengthexceeds || has_body_lengthexceeds || has_name_lengthexceeds {
    //         return HttpResponse::Forbidden()
    //             .body(include_str!("../../default_html/error_has_length_exceeds.html"));
    //     }

    //     if restriction::body_check(body, bbs_setting) || user.vacuum || user.level < bbs_setting.restriction_min_level {
    //         user.vacuum = true;
    //         let _ = user.update().await;

    //         let period_vacuum = bbs_setting.vacuum_period_sec;

    //         tokio::spawn(async move {
    //             sleep(Duration::from_secs(period_vacuum)).await;
    //             user.vacuum = false;
    //             let _ = user.update().await;
    //         });

    //         return HttpResponse::Forbidden()
    //             .body(include_str!("../../default_html/error_unknown.html"));
            
    //     }

    // }

    let (restriction, new_name) = post::post_filter(body, name, &bbspath.bbs_id);

    
    if (!title.is_empty() && !body.is_empty()) || !restriction {
        // ↓誤差の原因
        // user.level = user.level.saturating_sub(2);
        let _ = user.update().await;

        let topic = Topic::new(&data.title, password, None, &bbspath.bbs_id);

        match topic.commit().await {

            Ok(()) => {

                let post = Post::new(&new_name, body, &user);

                
                let _ = topic.post(post).await;

                let new_path = format!("{}/{}/topic/{}", &SETTING.base_url, &bbspath.bbs_id, &topic.topic_id);

                HttpResponse::MovedPermanently()
                    .append_header(("Location", new_path))
                    .finish() 
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