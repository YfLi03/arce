use std::path::{PathBuf};

pub type ArticleFolderList = Vec<ArticleFolder>;
pub type PictureFolderList = Vec<PictureFolder>;
pub struct ArticleFolder{
    pub path: PathBuf,
    pub need_confirm: bool
}

pub struct PictureFolder{
    pub path: PathBuf
}

