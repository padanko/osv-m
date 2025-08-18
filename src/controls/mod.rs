pub mod maketopic;
pub mod makepost;
pub mod reload;
pub mod user_info_migration;
pub mod headline;

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