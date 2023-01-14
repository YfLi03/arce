use std::path::PathBuf;
use exif::{In, Tag};
use imagesize::size;
use log::{info, warn};
use once_cell::sync::OnceCell;


use crate::api::err;
use crate::api::config;

use super::config::GlobalConfig;



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
    pub direction: String,
}

impl PhotographyPicture {
    fn calc_hash(&mut self) -> Result<(), err::Error> {
        let bytes = std::fs::read(&self.path)?;
        self.hash = sha256::digest(&*bytes);
        Ok(())
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

impl PhotographyPicture{
    pub fn read_info(mut self) -> Result<Self, err::Error>{
        
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
            },
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
            parameters +=  "  ";
        }
        if let Some(field) = exif.get_field(Tag::PhotographicSensitivity, In::PRIMARY) {
            parameters += "iso";
            parameters += &field.display_value().with_unit(&exif).to_string();
        }
        if let Some(field) = exif.get_field(Tag::Model, In::PRIMARY) {
            camera += &field.display_value().to_string();
            camera = camera.replacen("\"","",2);
        };

        self.params = parameters;
        self.camera = camera;
        self.date = date;

        Ok(self)
    }

    pub fn process_and_store(mut self) -> Result<Self, err::Error>{
        unimplemented!()
    }

    pub fn register_and_upload(&self) -> Result<(), err::Error> {
        let CONFIG = GlobalConfig::global();
        unimplemented!()
    }
}

