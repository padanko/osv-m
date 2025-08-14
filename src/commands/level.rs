use crate::models::user::User;

pub struct LevelView {
    tmp_val: i32
}

impl LevelView {
    pub fn new(user: &User) -> Self {
        Self { tmp_val: user.level as i32 }
    }
}

impl super::OsvCommand for LevelView {
    fn apply(&self, body_html: &str) -> String {
        body_html.replace("!ninja", &format!("<b class='green'>{} Lv</b>", self.tmp_val)).to_string()
    }
}

