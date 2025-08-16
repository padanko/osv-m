use std::{fs::File, io::Read, collections::HashMap};
use serde::{Serialize, Deserialize};
use serde_yaml;

#[derive(Serialize, Deserialize, Clone)]
pub struct BbsSetting {
    // 掲示板情報とか
    pub title: String,
    pub description_html: String,
    pub banner: String,
    
    // デフォルトネーム
    pub default_name: String,

    // 規制
    pub restriction_min_level: usize,
    pub restriction_handlename: bool,
    pub restriction_image: bool,
    pub vacuum_period_sec: u64,
    pub name_max_length: usize,
    pub body_max_length: usize,
    pub title_max_length: usize,
    pub restriction_words: Vec<String>,
    pub hide_id: bool,
    pub hide_link: bool,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ApplicationSetting {
    // サイト情報とか
    pub title: String,
    pub description: String,
    pub base_url: String,

    // ウェブサーバー
    pub bind_addr: String,
    pub bind_port: u16,
    pub cloudflare_dns_proxy: bool,

    // ユーザーID
    pub id_length: usize,
    pub id_charset: String,
    pub id_raw_prefix: String,
    
    // データベース
    pub postgresql_addr: String,
    pub postgresql_port: u16,
    pub postgresql_username: String,
    pub postgresql_password: String,
    pub postgresql_database: String,

    // 掲示板
    pub bbs: HashMap<String,BbsSetting>,
    pub html_folder: String,
    pub topic_id_digcount: Option<usize>,

    // フォーマット
    pub datetime_format: String,
    pub user_info_unknown_text: String,

    // コマンド
    pub enable_command: bool,
    pub rhai_ext_command: bool,
    pub rhai_exts_path: String
}

impl ApplicationSetting {
    pub fn from_file(filename: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let mut setting_file = File::open(filename)?;
        let mut setting_file_buffer = String::new();

        setting_file.read_to_string(&mut setting_file_buffer)?;

        let setting: ApplicationSetting = serde_yaml::from_str(&setting_file_buffer)?;
        
        Ok(setting)
    }
}

const SETTING_FILENAME: &str = "./osv-m_setting.yaml";

pub fn get_setting() -> Result<ApplicationSetting, Box<dyn std::error::Error>> {
    Ok(ApplicationSetting::from_file(SETTING_FILENAME)?)
}