use std::collections::{BinaryHeap, BTreeMap};
use std::str::FromStr;
use serde::Serialize;
use lazy_static::lazy_static;
use regex::Regex;
use std::time::{UNIX_EPOCH};
use std::fs::read_to_string;
use chrono::{NaiveDate};

use crate::markdown;

#[derive(Serialize)]
#[derive(Debug)]
#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub struct ArticleInfo{
    pub date: i64,
    pub title: String,
    pub name: String,   //the article name for url
    pub content: String,
}


fn read_with_yaml(raw_str: &str, content: &mut String) -> BTreeMap<String, String>{

    let mut config :BTreeMap<String, String> = BTreeMap::new();
    //finding the start loc
    if &raw_str[0..3] != "---" {
        content.push_str(&markdown::render_str_to_string(raw_str));
        return config;
    }

    let temp_str = &raw_str[3..raw_str.len()];  //jumping the first ---
    let pos = temp_str.find("---").expect("the markdown doc contains illegal yaml info");
    let md_str = &temp_str[pos+3..temp_str.len()];
    let yaml_str = &temp_str[0..pos];
    content.push_str(&markdown::render_str_to_string(md_str));
    //println!("{}",&yaml_str);
    config = serde_yaml::from_str(yaml_str).unwrap();
    //println!("{:?}",&config);
    config
}

/*  the function is used to read all articles, including about.md
    currently, about.html is rendered directly
    other contents are rendered to a String buffer
*/
pub fn read() -> Vec<ArticleInfo>{

    markdown::render("source/about.md","template/temp/about_content.html");
    /*  For now, all source pictures are stored in the public folder. 
        May need some change in the future.
        Articles are stored in the source/article folder    */
    
    let mut articles = BinaryHeap::new();
    let paths = std::fs::read_dir("./source/article").unwrap();
    let mut flag = false;

    for path in paths{
        let article_path = path.unwrap().path().display().to_string();

        
        //getting the dst url
        let mut name = "".to_string();
        lazy_static! {  //using lazy static to save compile time
            static ref RE: Regex = Regex::new(r"[/\\]([A-Za-z0-9_-]+)\.").unwrap();
        }
        for cap in RE.captures_iter(&article_path) {
            name = cap[1].to_string();
        }

        //doing something with the content
        let raw_str = read_to_string(&article_path)
            .expect("Cannot read markdown src");
        let mut content = String::new();
        let config = read_with_yaml(&raw_str,&mut content);
        
        let title;
        let mut date = 0;
        match config.get("title"){
            Some(t) => title = t.to_string(),
            None => title = name.clone()
        }
        match config.get("date"){
            Some(t) =>{
                let dt = NaiveDate::from_str(t)
                    .expect("Wrong date format")
                    .and_hms(0, 0, 0);
                //println!("{}",dt);
                date = dt.timestamp();
                if date < 0{
                    println!("Warning: The timestamp is \x1b[0;31m{}\x1b[0m. Make sure you have the yyyy-mm-dd format date.",date);
                }
            },
            None => {}
        }
        //let content = markdown::render_str_to_string(&md_str);
        
        //getting the date from metadata
        if date==0{
            let metadata = std::fs::metadata(&article_path).unwrap();
            if let Ok(time) = metadata.modified() {
                date = time.duration_since(UNIX_EPOCH).expect("Error Getting Time").as_secs().try_into().unwrap();
            } else {
                if !flag {
                    println!("Date Info Not supported on this platform and not found in article");
                    println!("It's recommended to contain date info within the article");
                    flag =  true;
                }
            }
        }
        
        let item = ArticleInfo{
            date,
            title,
            name,
            content,            
        };
        articles.push(item);
    }

    let mut vec = Vec::new();
    while !articles.is_empty(){
        vec.push(articles.pop().unwrap());
    }
    println!("Readed \x1b[0;31m{}\x1b[0m md files.",vec.len());
    vec
}