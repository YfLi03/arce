use std::path::PathBuf;
use once_cell::sync::OnceCell;
use crate::api::err;

pub struct GlobalConfig {
    pub title: String,
    pub subtitle: String,
    pub foot: String,
    pub bei_an: Option<String>,

    pub pic_cloud_prefix: String,
    pub pic_local: PathBuf,
    pub pic_compress_threshold: u64,

    pub scp_server: String,
    pub scp_pic_path: String,
    pub scp_web_path: String,

    pub deploy_auto: bool,
    pub deploy_interval: Option<usize>,
}

pub static CONFIG: OnceCell<GlobalConfig> = OnceCell::new();

impl GlobalConfig {
    pub fn global() -> &'static GlobalConfig {
        CONFIG.get().expect("Global Config is not initialized")
    }

    fn from_file() -> Result<GlobalConfig, err::Error> {
        unimplemented!()
    }
}