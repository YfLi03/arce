use std::path::PathBuf;

pub type PPictureList = Vec<PhotographyPicture>;
pub struct PhotographyPicture{
    pub hash_old: Option<String>,
    pub hash: String,
    pub path: PathBuf,

    pub selected: bool,

    pub title: String,
    pub params: String,
    pub date: String,
    pub camera: String,

    pub article_linked: bool,
    pub article_link: Option<String>,
}

pub struct Picture{
    pub hash_old: Option<String>,
    pub hash: String,
    pub path: PathBuf,
}

impl From<PhotographyPicture> for Picture{
    fn from(p: PhotographyPicture) -> Self {
        Picture { 
            hash_old: p.hash_old, 
            hash: p.hash, 
            path: p.path 
        }
    }
}