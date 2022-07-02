use std::{collections::{BinaryHeap,BTreeMap}, io::Write};
use serde::{Serialize,Deserialize};
use std::fs::{self};
use exif::{ In, Tag};
use imagesize::size;
use lazy_static::lazy_static;
use regex::Regex;
use image::io::Reader as ImageReader;

use crate::config::Config;

#[derive(Serialize)]
#[derive(Deserialize)]
#[derive(Default)]
#[derive(Debug)]
#[derive(PartialEq, Eq, PartialOrd, Ord)]
#[derive(Clone)]
pub struct PicInfo{
    date: String,
    url: String,
    title: String,
    parameters: String,
    camera: String,
    selected: bool,
    class: String,   //indicating the shape (Landscape, Portrait, Square)
    pic_size: u64   //sort of a hash key
}

fn read_pics_json() -> BTreeMap<String, PicInfo>{
    let mut map:BTreeMap<String,PicInfo> = BTreeMap::new();
    let json_str;
    match fs::File::open("./pics.json"){
        Ok(t) => json_str = t,
        Err(_e) =>{
            return map;
        }
    }
    let pics: Vec<PicInfo> = serde_json::from_reader(json_str)
            .expect("pics.yaml has a format error");
    for pic in pics{
        map.insert(pic.url.clone(), pic);
    }
    return map;
}

fn write_pics_json(map: &BTreeMap<String, PicInfo>){
    let mut pic: Vec<PicInfo> = Vec::new();
    for(_key, val) in map.iter(){
        pic.push(val.clone());
    }
    let json_str = serde_json::to_string(&pic).unwrap();
    let mut f = fs::File::create("./pics.json").unwrap();
    f.write(json_str.as_bytes()).unwrap();
}

fn read_pics(pic_list: &mut BinaryHeap<PicInfo>, s: String, is_selected: bool, compress: bool, existed: &mut BTreeMap<String, PicInfo>){
    let paths = fs::read_dir(s).unwrap();
    for path in paths{

        //read exif
        let pic_path = path.unwrap().path();
        let mut pic_size =  std::fs::metadata(&pic_path).unwrap().len();
        let file = std::fs::File::open(&pic_path).unwrap();
        let mut bufreader = std::io::BufReader::new(&file);
        let exifreader = exif::Reader::new();

        //basic info
        let mut url = String::from("gallery/");
        if is_selected {
            url += "selected/"
        }else{
            url+= "all/"
        }
        //let mut title = pic_path.file_name().unwrap().to_string_lossy().into_owned();
        url += &pic_path.file_name().unwrap().to_string_lossy();

        match existed.get(&url){
            Some(pic) => {
                //if the size matches, no longer need to read all the info
                if pic.pic_size == pic_size{
                    pic_list.push(pic.clone());
                    //println!("existed found");
                    continue;
                }
            },
            None => {}
        }
        
        //exif info
        let mut date = String::from("");
        let mut parameters = String::from("");
        let mut camera = String::from("");
        let mut class = String::from("");

        //read exif if exits
        match exifreader.read_from_container(&mut bufreader){
            Ok(exif) => {
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
            }
            Err(e) => {
                println!("Cannot read Exif \n {}",e);
            }
        }

        //get the title with regex
        let mut title = String::new();
        lazy_static! {  //using lazy static to save compile time
            static ref RE: Regex = Regex::new(r"([A-Za-z0-9_-]+)\.").unwrap();
        }
        for cap in RE.captures_iter(&pic_path.file_name().unwrap().to_string_lossy()) {
            title = cap[1].to_string();
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
            let mut image = ImageReader::open(&pic_path).unwrap().decode().unwrap();
            let filter = image::imageops::FilterType::Nearest;
            image = image.resize(1920,1920,filter);
            image.save(&pic_path).expect("Error saving the image");
            pic_size =  std::fs::metadata(&pic_path).unwrap().len();

        }

        //save the pic info
        let item = PicInfo{
            date,
            url:url.clone(),
            title,
            parameters,
            camera,
            selected:is_selected,
            class,
            pic_size
        };
        existed.insert(url, item.clone());
        pic_list.push(item);
    }
}

pub fn read(config: &Config) -> Vec<PicInfo>{
    let mut pic_list = BinaryHeap::new();
    let compress = if config.compress_image {true} else {false};
    let mut existed_pic = read_pics_json();


    read_pics(&mut pic_list, "./public/gallery/selected".to_string(), true, compress, &mut existed_pic);
    read_pics(&mut pic_list, "./public/gallery/all".to_string(), false, compress, &mut existed_pic);
    //let paths = fs::read_dir("./public/gallery/selected").unwrap();
    println!("\x1b[0;31m{}\x1b[0m pics readed", pic_list.len());
    if pic_list.len() == 0 {
        println!("\x1b[0;31mYou may need to add pictures to the /gallery/all and /gallery/selected folders\x1b[0m")
    }

    let mut pic_vec = Vec::new();
    while !pic_list.is_empty() {
        pic_vec.push(pic_list.pop().unwrap());
    }
    write_pics_json(&existed_pic);
    pic_vec
}