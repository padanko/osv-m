use crate::models::user::User;

pub trait OsvCommand {
    fn apply(&self, body_html: &str) -> String;
}

pub mod base;
pub mod level;

pub fn apply_all(body_html: &str, user: &User) -> String {
    let body_html = base::Rand::new().apply(body_html);
    let body_html = base::UrlAndImage::new().apply(&body_html);
    let body_html = level::LevelView::new(user).apply(&body_html);

    body_html
}