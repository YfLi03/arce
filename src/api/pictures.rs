use exif::{In, Tag};
use imagesize::size;
use log::{debug, info, warn};
use serde::Serialize;
use std::path::PathBuf;
use std::process::Command;

use crate::api::config::GlobalConfig;
use crate::api::err;
use crate::api::sync::GlobalConnPool;
use crate::model::pictures::{find_picture, insert_photography_picture, insert_picture};

pub type PPictureList = Vec<PhotographyPicture>;

#[derive(Default, Clone, Serialize, Debug)]
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
    pub direction: String,
}

impl PhotographyPicture {
    fn calc_hash(&mut self) -> Result<(), err::Error> {
        let bytes = std::fs::read(&self.path)?;
        self.hash = sha256::digest(&*bytes);
        Ok(())
    }

    pub fn is_registered(&mut self) -> Result<bool, err::Error> {
        self.calc_hash()?;
        let conn = GlobalConnPool::global().0.get().unwrap();
        let result = find_picture(&conn, &Picture::from(self.clone()))?;
        match result {
            None => Ok(false),
            _ => Ok(true),
        }
    }
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

#[derive(Debug)]
pub struct Picture {
    pub hash_old: Option<String>,
    pub hash: String,
    pub path: PathBuf,
}
impl Picture {
    pub fn from_dir(p: PathBuf) -> Result<Self, err::Error> {
        let bytes = std::fs::read(&p)?;
        let hash = sha256::digest(&*bytes);
        Ok(Picture {
            hash_old: None,
            hash,
            path: p,
        })
    }

    // both storing, registering and uploading
    pub fn register(mut self) -> Result<PathBuf, err::Error> {
        let config = GlobalConfig::global();
        let conn = GlobalConnPool::global().0.get().unwrap();
        let to = config.pic_local.join(
            self.hash.clone() + "." + self.path.clone().extension().unwrap().to_str().unwrap(),
        );
        if let Some(p) = find_picture(&conn, &self)? {
            return Ok(p);
        }
        std::fs::copy(&self.path, &to)?;
        self.path = to;

        let path = insert_picture(&conn, &self)?;
        self.path = path;

        let dst = config.scp_server.clone()
            + ":"
            + &config.scp_pic_path
            + "/"
            + self.path.clone().file_name().unwrap().to_str().unwrap();
        match Command::new("scp").arg(&self.path).arg(&dst).output() {
            Err(e) => {
                warn!("Upload of picture {:?} to {:?} failed.", &self.path, &dst);
            }
            _ => {}
        };
        Ok(self.path)
    }
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

impl PhotographyPicture {
    pub fn read_info(mut self) -> Result<Self, err::Error> {
        //height and width are not stored in exif.
        match size(&self.path) {
            Ok(r) => {
                if r.width == r.height {
                    self.direction = "Square".to_string();
                }
                if r.width > r.height {
                    self.direction = "Landscape".to_string();
                }
                if r.width < r.height {
                    self.direction = "Portrait".to_string();
                }
            }
            Err(err) => {
                warn!("size information for pic {:?} not found", self.path);
                return Ok(self);
            }
        };

        let mut file = std::fs::File::open(&self.path)?;
        let mut bufreader = std::io::BufReader::new(&file);
        let exifreader = exif::Reader::new();
        let exif = match exifreader.read_from_container(&mut bufreader) {
            Ok(exif) => exif,
            Err(e) => {
                warn!("exif information for pic {:?} not found", self.path);
                return Ok(self);
            }
        };

        let mut date = String::from("");
        let mut parameters = String::from("");
        let mut camera = String::from("");

        if let Some(field) = exif.get_field(Tag::DateTimeOriginal, In::PRIMARY) {
            date = field.display_value().with_unit(&exif).to_string();
        }
        if let Some(field) = exif.get_field(Tag::ExposureTime, In::PRIMARY) {
            parameters += &field.display_value().with_unit(&exif).to_string();
            parameters += "  ";
        }
        if let Some(field) = exif.get_field(Tag::FocalLengthIn35mmFilm, In::PRIMARY) {
            parameters += &field.display_value().with_unit(&exif).to_string();
            parameters += "  ";
        }
        if let Some(field) = exif.get_field(Tag::FNumber, In::PRIMARY) {
            parameters += &field.display_value().with_unit(&exif).to_string();
            parameters += "  ";
        }
        if let Some(field) = exif.get_field(Tag::PhotographicSensitivity, In::PRIMARY) {
            parameters += "iso";
            parameters += &field.display_value().with_unit(&exif).to_string();
        }
        if let Some(field) = exif.get_field(Tag::Model, In::PRIMARY) {
            camera += &field.display_value().to_string();
            camera = camera.replacen("\"", "", 2);
        };

        self.params = parameters;
        self.camera = camera;
        self.date = date;

        Ok(self)
    }

    pub fn process_and_store(mut self) -> Result<Self, err::Error> {
        let config = GlobalConfig::global();
        let size = std::fs::metadata(&self.path)?.len();
        if size < config.pic_compress_threshold {
            self.calc_hash()?;
            let to = config.pic_local.join(
                self.hash.clone() + "." + self.path.clone().extension().unwrap().to_str().unwrap(),
            );
            std::fs::copy(self.path, &to)?;
            self.path = to;
            return Ok(self);
        };

        info!("Compressing image {:?}", &self.path);
        let mut image = image::io::Reader::open(&self.path)?.decode()?;
        let filter = image::imageops::FilterType::Nearest;
        image = image.resize(1920, 1920, filter);

        let hash = sha256::digest(image.as_bytes());
        let save = config
            .pic_local
            .join(hash + "." + self.path.clone().extension().unwrap().to_str().unwrap());
        image.save(&save)?;
        self.path = save.clone();
        self.hash_old = Some(self.hash.clone());
        self.calc_hash()?;

        let to = config.pic_local.join(
            self.hash.clone() + "." + self.path.clone().extension().unwrap().to_str().unwrap(),
        );
        std::fs::rename(save, &to)?;
        self.path = to;

        Ok(self)
    }

    pub fn register_and_upload(&mut self) -> Result<(), err::Error> {
        let config = GlobalConfig::global();
        let conn = GlobalConnPool::global().0.get()?;
        insert_photography_picture(&conn, self)?;

        let dst = config.scp_server.clone()
            + ":"
            + &config.scp_pic_path
            + "/"
            + self.path.file_name().unwrap().to_str().unwrap();

        match Command::new("scp").arg(&self.path).arg(&dst).output() {
            Err(e) => {
                warn!("Upload of picture {:?} to {:?} failed.", &self.path, &dst);
            }
            _ => {}
        };
        Ok(())
    }
}

/// struct used for tera rendering
#[derive(Serialize, Clone)]
pub struct PhotographyPictureBrief {
    pub selected: bool,
    pub title: String,
    pub linked: bool,
    pub article_link: String,
    pub url: String,

    pub params: String,
    pub date: String,
    pub camera: String,
    pub direction: String,
}

impl From<PhotographyPicture> for PhotographyPictureBrief {
    fn from(p: PhotographyPicture) -> Self {
        debug!("Converting PP to PPB {:?}", p);
        PhotographyPictureBrief {
            selected: p.selected,
            title: p.title,
            linked: !(p.article_link == None),
            article_link: p.article_link.unwrap_or(String::new()),
            url: p.path.file_name().unwrap().to_str().unwrap().to_string(),
            params: p.params,
            date: p.date,
            camera: p.camera,
            direction: p.direction,
        }
    }
}
