pub mod maketopic;
pub mod makepost;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct BbsPath {
    pub bbs_id: String
}

#[derive(Deserialize)]
pub struct BbsTopicPath {
    pub bbs_id: String,
    pub topic_id: String,
}