use std::fs::read_to_string;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

use serde::{Deserialize, Serialize};

use crate::api::err;

pub type ArticleList = Vec<ArticleInfo>;

/// the article info stored in database
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

/// full article including contents
#[derive(Default, Serialize, Clone)]
pub struct Article {
    pub title: String,
    pub date: String,
    pub summary: String,
    pub url: String,
    pub category: String,
    pub headline: bool,
    pub content: String
}

#[derive(Deserialize)]
pub struct ArticleYaml {
    pub title: String,
    pub path: Option<String>,
    pub date: String,
    pub category: Option<String>,
    pub headline: Option<bool>,
    pub summary: Option<String>
}
/* 
// use as headline info
#[derive(Serialize)]
pub struct ArticleBrief {
    pub title: String,
    pub date: String,
    pub summary: String,
    pub url: String
}

impl From<Article> for ArticleBrief{
    fn from(a: Article) -> Self {
        ArticleBrief{
            title: a.title,
            date: a.date,
            summary: a.summary,
            url: a.url
        }
    }
}
*/
#[derive(Serialize)]
pub struct CategoryBrief{
    pub title: String,
    pub summary: String,
    pub url: String
}

pub fn find_deploy_flag(path: &PathBuf) -> Result<bool, err::Error> {
    Ok(read_to_string(path)?.find("deploy: true").is_some())
}
// maybe some impl
