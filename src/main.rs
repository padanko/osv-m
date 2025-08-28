// OSV-M
// 複数板運営可能なOSV

use actix_web::{
    web::{get, post}, App, HttpServer,
    web::Data,
};

use std::sync::Mutex;

use tokio;
use tera::Tera;

mod extension;
mod utils;
mod views;
mod controls;
mod models;

// エクスクラメーションマークから始まるコマンド群
mod commands;

use once_cell::sync::Lazy;

const VERSION: &'static str = env!("CARGO_PKG_VERSION");

// #[cfg(test)]
// const SETTING: Lazy<utils::setting::ApplicationSetting> = Lazy::new(|| {

//     let setting = utils::setting::ApplicationSetting::from_file("./test/setting.yaml").expect("設定を取得できませんでした");

//     setting
// });

// #[cfg(not(test))]
const SETTING: Lazy<utils::setting::ApplicationSetting> = Lazy::new(|| {

    let setting = utils::setting::get_setting().expect("設定を取得できませんでした");

    setting
});

// ロングポーリング用

#[derive(Clone)]
struct PollState {
    randvalue: String,
    bbs_id: String,
    topic_id: String
}

static POLL_INSTANCE: Lazy<Mutex<PollState>> = Lazy::new(|| {
    Mutex::new(PollState { 
        randvalue: String::new(),
        bbs_id: String::new(),
        topic_id: String::new()
    })
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
            .route("/dev/userinfo/self", get().to(views::user_info::endpoint_self))
            .route("/dev/change-ip-addr", post().to(controls::user_info_migration::endpoint))
            .route("/{bbs_id}/", get().to(views::bbs::endpoint))
            .route("/{bbs_id}/topic/{topic_id}", get().to(views::topic::endpoint))
            .route("/{bbs_id}/make/topic", post().to(controls::maketopic::endpoint))
            .route("/{bbs_id}/make/post/{topic_id}", post().to(controls::makepost::endpoint))
            .route("/{bbs_id}/get/headline", get().to(controls::headline::endpoint))
            .route("/{bbs_id}/poll/{topic_id}", get().to(controls::reload::endpoint))
    })
        .bind(format!("{}:{}", SETTING.bind_addr, SETTING.bind_port))?
        .run()
        .await?;
    Ok(())
}