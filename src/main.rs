// OSV-M
// 複数板運営可能なOSV

use actix_web::{
    web::{get, post}, App, HttpServer,
    web::Data,
};

use tokio;
use tera::Tera;

mod utils;
mod views;
mod controls;
mod models;

use once_cell::sync::Lazy;

const SETTING: Lazy<utils::setting::ApplicationSetting> = Lazy::new(|| {
    let setting = utils::setting::get_setting().expect("Error");

    setting
});



#[derive(Clone)]
struct ActixWebData {
    tera: Tera,
}


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    let tera = Tera::new(&SETTING.html_folder)?;

    let actix_web_data = ActixWebData {
        tera: tera
    };

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(actix_web_data.clone()))
            .route("/", get().to(views::index::endpoint))
            .route("/bbstable/", get().to(views::bbstable::endpoint))
            .route("/{bbs_id}/", get().to(views::bbs::endpoint))
            .route("/{bbs_id}/topic/{topic_id}", get().to(views::topic::endpoint))
            .route("/{bbs_id}/make/topic", post().to(controls::maketopic::endpoint))
            .route("/{bbs_id}/make/post/{topic_id}", post().to(controls::makepost::endpoint))
            
    })
        .bind(format!("{}:{}", SETTING.bind_addr, SETTING.bind_port))?
        .run()
        .await?;
    Ok(())
}