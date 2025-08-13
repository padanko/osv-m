use actix_web::{web::{Data}, HttpResponse, Responder};
use crate::{ActixWebData, SETTING};

use tera::Context;

// GET /
pub async fn endpoint(data: Data<ActixWebData>) -> impl Responder {

    let mut ctx = Context::new();

    ctx.insert("title", &SETTING.title);
    ctx.insert("description", &SETTING.description);
    ctx.insert("bbs_num", &SETTING.bbs.len());
    ctx.insert("baseurl", &SETTING.base_url);
    

    match data.tera.render("index.html", &ctx) {
        Ok(html) => {
            HttpResponse::Ok()
                .body(html)
        },
        Err(_) => {
            HttpResponse::InternalServerError()
                .body(include_str!("../../default_html/error_render.html"))
        }
    }

}