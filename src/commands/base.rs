use rand::random_range;

use crate::SETTING;

/////////////////////////////////////

pub struct Rand;

impl Rand {
    pub fn new() -> Self {
        Self { }
    }
}

impl super::OsvCommand for Rand {
    fn apply(&self, body_html: &str) -> String {
        let mut body = body_html.to_string();
        for _ in 0..body_html.matches("!random").count() {
            body = body.replacen("!random", &format!("<b class='random'>{}</b>", random_range(0..=100)), 1);
        }

        for _ in 0..body_html.matches("!rand").count() {
            body = body.replacen("!rand", &format!("<b class='random'>{}</b>", random_range(0..=100)), 1);
        }

        body.to_string()
    }
}

/////////////////////////////////////

pub struct UrlAndImage {
    bbs_id: String
}

impl UrlAndImage {
    pub fn new(bbs_id: &str) -> Self {
        Self {
            bbs_id: bbs_id.to_string()
        }
    }
}

impl super::OsvCommand for UrlAndImage {
    fn apply(&self, body_html: &str) -> String {
        if let Some(bbs_setting) = SETTING.bbs.get(&self.bbs_id) {
            let mut lines: Vec<String> = Vec::new();

            for line in body_html.lines() {
                if line.starts_with("!URL:") {
                    let args_: String = line.chars().skip(5).collect();
                    lines.push(format!("<a href='{args_}' target='blank_'>{args_}</a>"));
                } else if line.starts_with("!Img:") && !bbs_setting.restriction_image {
                    let args_: String = line.chars().skip(5).collect();
                    lines.push(format!("<img src='{}' class='image-post'>", args_));
	            } else if line.starts_with("https://i.imgur.com/") && !bbs_setting.restriction_image {
		            // Open2ch方式
                    let args_: String = line.to_string();
                    lines.push(format!("<img src='{}' class='image-post'>", args_));
                } else if line.starts_with("https://") && !bbs_setting.restriction_image {
                    let args_: String = line.to_string();
                    lines.push(format!("<a href='{}'>{}</a>", args_, args_));
		        } else {
                    lines.push(line.to_string());
                }
            }

            lines.join("\n")
        } else {
            body_html.to_string()
        }
    }
}

