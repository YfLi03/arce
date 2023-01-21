use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

use serde::{Deserialize, Serialize};

pub type ArticleList = Vec<ArticleInfo>;

/// the article info stored in database
pub struct ArticleInfo {
    pub path: PathBuf,
    pub deploy_folder: String, // Full Path of Deployment
    pub time: u64,             // Unix Timestamp
}

impl ArticleInfo {
    /// generate an articleinfo variable with its path and deploy folder
    pub fn new(p: PathBuf, s: String) -> Self {
        ArticleInfo {
            path: p,
            time: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            deploy_folder: s,
        }
    }
}

/// full article including contents
#[derive(Default, Serialize, Clone, Debug)]
pub struct Article {
    pub title: String,
    pub date: String,
    pub summary: String,
    pub url: String,
    pub category: String,
    pub headline: bool,
    pub content: String,
}

/// Struct used for parsing Yaml Front Matter in articles
#[derive(Deserialize)]
pub struct ArticleYaml {
    pub title: String,
    pub path: Option<String>,
    pub date: String,
    pub category: Option<String>,
    pub headline: Option<bool>,
    pub summary: Option<String>,
}
