pub trait OsvCommand {
    fn apply(&self, body_html: &str) -> String;
}

pub mod base;

pub fn apply_all(body_html: &str) -> String {
    let body_html = base::Rand::new().apply(body_html);
    let body_html = base::UrlAndImage::new().apply(&body_html);

    body_html
}