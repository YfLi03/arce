/*
    Read the pictures and get their info
*/
use std::{collections::{BinaryHeap,BTreeMap, HashSet}, io::Write};
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

/*
    The following two functions are written for the Persistance of data
    When a pic's info is read, it'll be written to pics.json
    So the next time, if the program found that the pic's name exists in the pics.json
    And they have the same size, they'll be regarded as the same file
    The info in pics.json will be used directly instead of reading again 
*/
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
            .expect("pics.json has a format error");
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

//reading a specific folder
fn read_pics(pic_list: &mut BinaryHeap<PicInfo>, s: String, is_selected: bool, compress: bool, existed: &mut BTreeMap<String, PicInfo>, article_name_set:&HashSet<String>){

    let paths = fs::read_dir(s).unwrap();
    for path in paths{

        //read basic info
        let pic_path = path.unwrap().path();
        let mut pic_size =  std::fs::metadata(&pic_path).unwrap().len();
        let file = std::fs::File::open(&pic_path).unwrap();
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
        match existed.get(&url){
            Some(pic) => {
                //if the size matches, no longer need to read all the info
                if pic.pic_size == pic_size{
                    //but the link info needs to be updated
                    let mut item = pic.clone();
                    item.link = link;
                    item.has_link = has_link;

                    pic_list.push(item);
                    //println!("existed found");
                    continue;
                }
            },
            None => {}
        }
        
        //get the exif info ( if exists )
        let mut date = String::from("");
        let mut parameters = String::from("");
        let mut camera = String::from("");
        let mut class = String::from("");

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
            name,
            parameters,
            camera,
            selected:is_selected,
            class,
            pic_size,
            link,
            has_link
        };
        existed.insert(url, item.clone());
        pic_list.push(item);
    }
}

pub fn read(config: &Config, article_name_set:&HashSet<String>) -> Vec<PicInfo>{
    let mut pic_list = BinaryHeap::new();
    let compress = if config.compress_image {true} else {false};
    let mut existed_pic = read_pics_json();


    read_pics(&mut pic_list, "./public/gallery/selected".to_string(), true, compress, &mut existed_pic, &article_name_set);
    read_pics(&mut pic_list, "./public/gallery/all".to_string(), false, compress, &mut existed_pic, &article_name_set);
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