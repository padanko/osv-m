use super::setting::BbsSetting;

pub fn body_check(body: &str, setting: &BbsSetting) -> bool{
    for word in &setting.restriction_words {
        if body.contains(word) {
            return true;
        }
    }

    false
}