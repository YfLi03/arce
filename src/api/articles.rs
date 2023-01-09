use std::path::PathBuf;

pub struct ArticleInfo{
    pub path: PathBuf,
    pub deploy_path: PathBuf,
    pub time: u64   // Unix Timestamp
}