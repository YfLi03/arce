/*
Generate the articles (markdown to html str), and the about page
*/

use std::collections::{BinaryHeap, BTreeMap, HashSet};
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
    pub date: i64,      //linux epoch time in seconds
    pub title: String,  //the title shown
    pub name: String,   //the article url, with no suffix
    pub content: String,//content(html)
}

//some articles has yaml info on the top
//returns the yaml settings, and also the rendered content
fn read_with_yaml(raw_str: &str, content: &mut String) -> BTreeMap<String, String>{

    let mut config :BTreeMap<String, String> = BTreeMap::new();
    //finding the start loc
    if &raw_str[0..3] != "---" {
        //if no yaml is found, return directly
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

//the main function
//the vec contains the articles, while the name_set is used to determine whether a picture
//with the same name is found
pub fn read(name_set: &mut HashSet<String>) -> Vec<ArticleInfo>{

    markdown::render("source/about.md","template/temp/about_content.html");
    /*  For now, all source pictures are stored in the public folder.
        Articles are stored in the source/article folder    */
    
    //using a binaryheap, because articles should be displayed in time order
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
        name_set.insert(name.clone());

        //doing something with the content
        let raw_str = read_to_string(&article_path)
            .expect("Cannot read markdown src");
        let mut content = String::new();
        let config = read_with_yaml(&raw_str,&mut content);
        
        //dealing with the arrtibutes of the article
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
        
        //if not found in yaml, get the date from metadata
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

    //returns an ordered vec
    let mut vec = Vec::new();
    while !articles.is_empty(){
        vec.push(articles.pop().unwrap());
    }
    println!("Readed \x1b[0;31m{}\x1b[0m md files.",vec.len());
    vec
}