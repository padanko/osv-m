use actix_web::{web::Data, HttpResponse, Responder};
use serde::Serialize;
use crate::{ActixWebData, SETTING};

use tera::Context;

// GET /bbstable/

#[derive(Serialize)]
pub struct BbsList {
    pub bbsname: String,
    pub bbsid: String
}

pub async fn endpoint(data: Data<ActixWebData>) -> impl Responder {

    let mut ctx = Context::new();

    let setting = &SETTING;

    let mut bbslist: Vec<&String> = setting.bbs.keys().collect();

    bbslist.sort();

    let mut bbslist_: Vec<BbsList> = Vec::new();

    for bbsid in bbslist {
        if let Some(bbs_setting) = setting.bbs.get(bbsid) {
            bbslist_.push(BbsList {
                bbsname: bbs_setting.title.to_string(),
                bbsid: bbsid.to_string()
            })        
        }
    }


    ctx.insert("bbslist", &bbslist_);
    ctx.insert("title", &setting.title);
    ctx.insert("baseurl", &SETTING.base_url);

    match data.tera.render("bbstable.html", &ctx) {
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