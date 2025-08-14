use rand::random_range;

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

        body.to_string()
    }
}

/////////////////////////////////////

pub struct UrlAndImage;

impl UrlAndImage {
    pub fn new() -> Self {
        Self { }
    }
}

impl super::OsvCommand for UrlAndImage {
    fn apply(&self, body_html: &str) -> String {
        let mut lines: Vec<String> = Vec::new();

        for line in body_html.lines() {
            if line.starts_with("!URL:") {
                let args_: String = line.chars().skip(5).collect();
                lines.push(format!("<a href='{args_}' target='blank_'>{args_}</a>"));
            } else if line.starts_with("!Img:") {
                let args_: String = line.chars().skip(5).collect();
                lines.push(format!("<img src='{}' class='image-post'>", args_));
            } else {
                lines.push(line.to_string());
            }
        }

        lines.join("\n")
    }
}

