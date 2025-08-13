use actix_web::HttpRequest;

use crate::SETTING;

pub fn get_ipaddr_from_header(req: &HttpRequest) -> Option<String> {
    let header = req.headers();

    let key = if SETTING.cloudflare_dns_proxy {
        "CF-Connecting-IP"
    } else {
        "X-Forwarded-For"
    };

    match header.get(key) {
        Some(value) => {
            if let Ok(value_str) = value.to_str() {
                Some(value_str.to_string())
            } else {
                Some(String::from("???"))
            }
        },
        None => {
            None
        }
    }
}