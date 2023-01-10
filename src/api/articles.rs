use std::path::PathBuf;

pub type ArticleList = Vec<ArticleInfo>;

pub struct Article{

}

pub struct ArticleInfo{
    pub path: PathBuf,
    pub deploy_path: String,
    pub time: u64   // Unix Timestamp
}

pub struct ArticleBrief{

}

// maybe some impl