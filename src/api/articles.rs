use std::fs::read_to_string;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::api::err;

pub type ArticleList = Vec<ArticleInfo>;

/// full article including contents
pub struct Article {}

pub struct ArticleInfo {
    pub path: PathBuf,
    pub deploy_folder: String, // Full Path of Deployment
    pub time: u64,             // Unix Timestamp
}

impl From<(PathBuf, String)> for ArticleInfo {
    fn from(p: (PathBuf, String)) -> Self {
        ArticleInfo {
            path: p.0,
            time: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            deploy_folder: p.1,
        }
    }
}

// use as headline info
pub struct ArticleBrief {}

pub fn find_deploy_flag(path: &PathBuf) -> Result<bool, err::Error> {
    Ok(read_to_string(path)?.find("deploy: true").is_some())
}
// maybe some impl
