use std::collections::BinaryHeap;
use serde::Serialize;
use lazy_static::lazy_static;
use regex::Regex;
use std::time::{UNIX_EPOCH};

use crate::markdown;

#[derive(Serialize)]
#[derive(Debug)]
#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub struct ArticleInfo{
    date: u64,
    pub name: String,
    pub content: String,
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


        let content = markdown::render_to_string(&article_path);
        
        //getting the date
        let mut date = 0;
        let metadata = std::fs::metadata(&article_path).unwrap();
        if let Ok(time) = metadata.modified() {
            date = time.duration_since(UNIX_EPOCH).expect("Error Getting Time").as_secs();
        } else {
            if !flag {
                println!("Date Info Not supported on this platform or filesystem");
                println!("May not be able to order articles");
                flag =  true;
            }
        }
        

        let item = ArticleInfo{
            date,
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