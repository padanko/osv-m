use crate::models::user::User;

pub trait OsvCommand {
    fn apply(&self, body_html: &str) -> String;
}

pub mod base;
pub mod level;

pub fn apply_all(body_html: &str, user: &User, bbs_id: &str) -> String {
    let body_html = base::Rand::new().apply(body_html);
    let body_html = base::UrlAndImage::new(user, bbs_id).apply(&body_html);
    let body_html = level::LevelView::new(user).apply(&body_html);

    body_html
}