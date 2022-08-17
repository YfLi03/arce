/*
    Read the pictures and get their info
*/
use std::{collections::{BinaryHeap, HashSet}, error::Error};
use serde::{Serialize,Deserialize};
use rusqlite::{types::ToSql, Connection};
use std::fs::{self};
use exif::{In, Tag};
use imagesize::size;
use lazy_static::lazy_static;
use regex::Regex;
use image::io::Reader as ImageReader;

use crate::config::Config;
use crate::sql;

#[derive(Serialize)]
#[derive(Deserialize)]
#[derive(Default)]
#[derive(Debug)]
#[derive(PartialEq, Eq, PartialOrd, Ord)]
#[derive(Clone)]
pub struct PicInfo{
    date: String,   //date in string (to display and sort)
    url: String,    //url with suffix
    name: String,   //name of the picture (to display)
    parameters: String, //iso, shutter speed...
    camera: String,
    selected: bool,   
    class: String,   //indicating the shape (Landscape, Portrait, Square)
    pic_size: u64,   //used as a hash key to determine whether it's the same pic
    has_link: bool,  //if it has an article with the same name, a link will be generated automatically
    link: String,   // link to the corresponding image
}

impl PicInfo{
    pub fn new(url: String, name: String, pic_size: u64, date: String, parameters: String, camera: String,
    selected: bool, class: String, has_link: bool, link: String) -> PicInfo{
        PicInfo{
            date,
            url,
            name,
            parameters,
            camera,
            selected,
            class,
            pic_size,
            has_link,
            link
        }
    }

    pub fn get_sql(&self) -> [&dyn ToSql; 10]{
        [&self.url as &dyn ToSql, &self.name, &self.pic_size, &self.date, &self.parameters, &self.camera,
        &self.selected, &self.class, &self.has_link, &self.link]
    }
}


//reading a specific folder
fn read_pics(pic_list: &mut BinaryHeap<PicInfo>, s: String, is_selected: bool, compress: bool,  article_name_set:&HashSet<String>, db: &Connection) -> Result<(), Box<dyn Error>>{

    let paths = fs::read_dir(s)?;
    for path in paths{

        //read basic info
        let pic_path = path?.path();
        let mut pic_size =  std::fs::metadata(&pic_path)?.len();
        let file = std::fs::File::open(&pic_path)?;
        let mut bufreader = std::io::BufReader::new(&file);
        let exifreader = exif::Reader::new();


        //get the name(with no suffix) with regex
        let mut name = String::new();
        lazy_static! {  //using lazy static to save compile time
            static ref RE: Regex = Regex::new(r"([A-Za-z0-9_-]+)\.").unwrap();
        }
        for cap in RE.captures_iter(&pic_path.file_name().unwrap().to_string_lossy()) {
            name = cap[1].to_string();
        }

        //determine if there's an article with the same name
        let link;
        let has_link;
        if article_name_set.contains(&name){
            link = "/articles/".to_string()+&name+".html";
            has_link = true;
        }else{
            link = String::new();
            has_link = false;
        }

        //get the url
        let mut url = String::from("gallery/");
        if is_selected {
            url += "selected/"
        }else{
            url+= "all/"
        }
        //let mut name = pic_path.file_name().unwrap().to_string_lossy().into_owned();
        url += &pic_path.file_name().unwrap().to_string_lossy();

        //if the pic's info is found in pics.json
        match sql::query(&db, &url, pic_size)?{
            Some(mut pic) => {
                pic.link = link;
                pic.has_link = has_link;
                //Some Info need to be updated
                pic_list.push(pic);
                continue;
            },
            None => {}
        }
        
        //get the exif info ( if exists )
        let mut date = String::from("");
        let mut parameters = String::from("");
        let mut camera = String::from("");
        let mut class = String::from("");

        let exif = match exifreader.read_from_container(&mut bufreader){
            Ok(exif) => exif,
            Err(err) => {
                println!("{} for {}",err, url);
                continue;
            }
        };
            
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
        }

        //height and width are not stored in exif.
        match size(&pic_path) {
            Ok(r) => {
                if r.width == r.height {
                    class = "Square".to_string();
                }
                if r.width > r.height {
                    class = "Landscape".to_string();
                }
                if r.width < r.height {
                    class = "Portrait".to_string();
                }
            }
            Err(err) => println!("Error getting size: {:?}", err)
        }

        //compress the image if it's too large
        //sadly, this will lead to losing exif
        if pic_size > 800000 && compress {
            println!("file \x1b[0;31m{:?}\x1b[0m will be compressed", &pic_path);
            println!("May take some time");

            //rust's image-rs seems to be very slow
            let mut image = ImageReader::open(&pic_path)?.decode()?;
            let filter = image::imageops::FilterType::Nearest;
            image = image.resize(1920,1920,filter);
            image.save(&pic_path).expect("Error saving the image");
            pic_size =  std::fs::metadata(&pic_path).unwrap().len();

        }

        //save the pic info
        let item = PicInfo{
            date,
            url:url.clone(),
            name,
            parameters,
            camera,
            selected:is_selected,
            class,
            pic_size,
            link,
            has_link
        };
        sql::insert(&db, &item)?;
        //existed.insert(url, item.clone());
        pic_list.push(item);
    }
    Ok(())
}

pub fn read(config: &Config, article_name_set:&HashSet<String>) -> Result<Vec<PicInfo>, Box<dyn Error>>{
    let mut pic_list = BinaryHeap::new();
    let compress = if config.compress_image {true} else {false};
    //let mut existed_pic = read_pics_json();
    let conn = sql::connect()?;

    read_pics(&mut pic_list, "./public/gallery/selected".to_string(), true, compress, &article_name_set, &conn)?;
    read_pics(&mut pic_list, "./public/gallery/all".to_string(), false, compress, &article_name_set, &conn)?;
    //let paths = fs::read_dir("./public/gallery/selected").unwrap();
    
    println!("\x1b[0;31m{}\x1b[0m pics readed", pic_list.len());
    if pic_list.len() == 0 {
        println!("\x1b[0;31mYou may need to add pictures to the /gallery/all and /gallery/selected folders\x1b[0m")
    }

    let mut pic_vec = Vec::new();
    while !pic_list.is_empty() {
        pic_vec.push(pic_list.pop().unwrap());
    }
    //write_pics_json(&existed_pic);
    Ok(pic_vec)
}