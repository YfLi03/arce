use std::collections::BinaryHeap;
use serde::Serialize;
use std::fs::{self};
use exif::{ In, Tag};
use imagesize::size;

#[derive(Serialize)]
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
    class: String   //indicating the shape (Landscape, Protrait, Square)
}

fn read_pics(pic_list: &mut BinaryHeap<PicInfo>, s: String, is_selected: bool){
    let paths = fs::read_dir(s).unwrap();
    for path in paths{

        //read exif
        let pic_path = path.unwrap().path();
        let file = std::fs::File::open(&pic_path).unwrap();
        let mut bufreader = std::io::BufReader::new(&file);
        let exifreader = exif::Reader::new();
        let exif = exifreader.read_from_container(&mut bufreader).unwrap();


        /*
        for f in exif.fields() {
            println!("{} {} {}",
                     f.tag, f.ifd_num, f.display_value().with_unit(&exif));
        }
        */
    
        //process exif  
        let mut date = String::from("");
        let mut parameters = String::from("");
        let mut camera = String::from("");
        let mut class = String::from("");

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

        let mut url = String::from("gallery/");
        if is_selected {
            url += "selected/"
        }else{
            url+= "all/"
        }
        let title = pic_path.file_name().unwrap().to_string_lossy().into_owned();
        url += &title;


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

        //save the pic 
        let item = PicInfo{
            date,
            url,
            title,
            parameters,
            camera,
            selected:is_selected,
            class,
        };
        pic_list.push(item);
    }
}

pub fn read() -> BinaryHeap<PicInfo>{
    let mut pic_list = BinaryHeap::new();
    read_pics(&mut pic_list, "./public/gallery/selected".to_string(), true);
    read_pics(&mut pic_list, "./public/gallery/all".to_string(), false);
    //let paths = fs::read_dir("./public/gallery/selected").unwrap();
    println!("\x1b[0;31m{}\x1b[0m pics readed", pic_list.len());
    if pic_list.len() == 0 {
        println!("\x1b[0;31mYou may need to add pictures to the /gallery/all and /gallery/selected folders\x1b[0m")
    }
    pic_list
}