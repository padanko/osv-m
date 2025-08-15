use std::io::Write;

use crate::models::user::User;
use sha2::{Digest, Sha256};

pub struct LevelView {
    tmp_val: i32,
    token_sha256: String
}


impl LevelView {
    pub fn new(user: &User) -> Self {

        let mut hasher = Sha256::new();
        
        let _ = hasher.write((&user.token).as_bytes());
        let _ = hasher.flush();

        let x = hasher.finalize().to_vec();

        let hex = if let Some(x) = x.get(0..8) {
            let mut hex_ = String::new();
            for byte in x {
                hex_ += &format!("{:x}", byte);
            }

            format!("0x{}", hex_)
        } else {
            String::from("???")
        };

        Self {
            tmp_val: user.level as i32,
            token_sha256: hex 
        }
    }
}

impl super::OsvCommand for LevelView {
    fn apply(&self, body_html: &str) -> String {
        let html = body_html.replace("!ninja", &format!("<b class='green'>{} Lv</b>", self.tmp_val/10)).to_string();
        let html = html.replace("!hash", &format!("<b class='green'>{}</b>", self.token_sha256));
        html
    }
}