use actix_web::{HttpRequest, Responder, HttpResponse};
use actix_web::web::{Data, Form};
use serde::Deserialize;
use tera::Context;
use crate::models::user::User;
use crate::utils::get_ip::get_ipaddr_from_header;
use crate::SETTING;

#[derive(Deserialize)]
pub struct MigrationRequest {
    pub user_token: String
}

pub async fn endpoint(req: HttpRequest, form_: Form<MigrationRequest>, data: Data<crate::ActixWebData>) -> impl Responder {
    
    let mut ctx = Context::new();
    let tera = &data.tera;
    
    ctx.insert("title", &SETTING.title);
    ctx.insert("baseurl", &SETTING.base_url);

    let html = if let Some(ip_addr) = get_ipaddr_from_header(&req) {
        match (User::from_token(&form_.user_token).await, User::from(&ip_addr).await) {
            (Ok(user), Ok(mut new_user)) => {
                if let Err(e) = new_user.migration_userip(&ip_addr, &user.token).await {
                    eprintln!("エラー\t\t{}", e);
                    tera.render("user/fail_userdata_migration.html", &ctx)
                } else {
                    tera.render("user/success_userdata_migration.html", &ctx)
                }
            },
            (error1, error2) => {
                if let Err(error) = error1 {
                    eprintln!("エラー\t\t{}", error);
                }

                if let Err(error) = error2 {
                    eprintln!("エラー\t\t{}", error);
                }

                tera.render("user/fail_userdata_migration.html", &ctx)
            }
        }
    } else {
        tera.render("user/fail_userdata_migration.html", &ctx)
    };

    match html {
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