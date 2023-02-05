use crate::api::err;

use log::info;
use once_cell::sync::OnceCell;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// The global Config Struct
#[derive(Debug, Deserialize, Serialize)]
pub struct GlobalConfig {
    pub title: String,
    pub subtitle: String,
    pub foot: String,
    pub bei_an: Option<String>,
    pub url: String,
    pub robot: Option<String>,

    pub pic_cloud_prefix: String,
    pub pic_local: PathBuf,
    pub pic_compress_threshold: u64,
    pub pic_replace_prefix: String,

    pub scp_server: String,
    pub scp_pic_path: String,
    pub scp_web_path: String,

    pub deploy_auto: bool,
    pub deploy_interval: Option<u64>,
}

/// OnceCell variable to make it static
pub static CONFIG: OnceCell<GlobalConfig> = OnceCell::new();

impl GlobalConfig {
    pub fn global() -> &'static GlobalConfig {
        CONFIG.get().expect("Global Config is not initialized")
    }

    // reading the config file
    pub fn from_file(f: PathBuf) -> Result<GlobalConfig, err::Error> {
        let yaml = std::fs::File::open(f)?;
        let config: GlobalConfig = serde_yaml::from_reader(yaml)?;
        info!("CONFIG READ {:?}", &config);
        Ok(config)
    }
}
