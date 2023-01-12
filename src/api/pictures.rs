use std::path::PathBuf;

use crate::api::err;

pub type PPictureList = Vec<PhotographyPicture>;

#[derive(Default)]
pub struct PhotographyPicture {
    pub hash_old: Option<String>,
    pub hash: String,
    pub path: PathBuf,

    pub selected: bool,
    pub title: String,
    pub article_link: Option<String>,

    pub params: String,
    pub date: String,
    pub camera: String,
}

impl PhotographyPicture {
    pub fn from_dir(
        path: PathBuf,
        selected: bool,
        article_link: Option<String>,
        title: String,
    ) -> Result<Self, err::Error> {
        let bytes = std::fs::read(&path)?;
        let hash = sha256::digest(&*bytes);
        Ok(PhotographyPicture {
            path,
            selected,
            article_link,
            hash,
            title,
            ..Default::default()
        })
    }
}

pub struct Picture {
    pub hash_old: Option<String>,
    pub hash: String,
    pub path: PathBuf,
}

impl From<PhotographyPicture> for Picture {
    fn from(p: PhotographyPicture) -> Self {
        Picture {
            hash_old: p.hash_old,
            hash: p.hash,
            path: p.path,
        }
    }
}

pub fn read_info(p: PhotographyPicture) -> PhotographyPicture {
    unimplemented!()
}
