use actix_web::{web::{Data}, HttpResponse, Responder, HttpRequest};
use crate::{models, utils, ActixWebData, SETTING};

use tera::Context;

// GET /dev/user/self
pub async fn endpoint_self(req: HttpRequest, data: Data<ActixWebData>) -> impl Responder {

    let mut ctx = Context::new();

    ctx.insert("title", &SETTING.title);
    ctx.insert("baseurl", &SETTING.base_url);
    
    if let Some(ip_addr) = utils::get_ip::get_ipaddr_from_header(&req) {
        ctx.insert("ip_addr", &ip_addr);
        ctx.insert("user_id", &utils::random_id::generate_user_view_id(&ip_addr));
        match models::user::User::new(&ip_addr).await {
            Ok(user) => {
                ctx.insert("user_vacuum", &user.vacuum);
                ctx.insert("user_token", &user.token);
                ctx.insert("user_postcount", &user.level);
            },
            Err(_) => {
                ctx.insert("user_vacuum", &SETTING.user_info_unknown_text);
                ctx.insert("user_token", &SETTING.user_info_unknown_text);
                ctx.insert("user_postcount", &SETTING.user_info_unknown_text);
            }
        }
    } else {
        ctx.insert("ip_addr", &SETTING.user_info_unknown_text);
        ctx.insert("user_id", &SETTING.user_info_unknown_text);
        ctx.insert("user_vacuum", &SETTING.user_info_unknown_text);
        ctx.insert("user_token", &SETTING.user_info_unknown_text);
        ctx.insert("user_postcount", &SETTING.user_info_unknown_text);
    }

    match data.tera.render("user/info.html", &ctx) {
        Ok(html) => {
            HttpResponse::Ok()
                .body(html)
        },
        Err(e) => {
            eprintln!("{}", e);
            HttpResponse::InternalServerError()
                .body(include_str!("../../default_html/error_render.html"))
        }
    }

}