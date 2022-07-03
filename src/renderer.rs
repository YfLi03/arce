use tera::Tera;
use tera::Context;
use std::fs::File;
use std::io::Write;
use serde::Serialize;
use chrono::prelude::*;

use crate::config::Config;
use crate::pic_selector::PicInfo;
use crate::article::ArticleInfo;

#[derive(Serialize)]
#[derive(Debug)]
struct HeaderConfig(String, String, String);


#[derive(Serialize)]
struct NavConfig{
    has_prev: bool,
    has_next: bool,
    next: String,
    prev: String,
    next_text: String,
    prev_text: String
}

fn render(tera: &Tera, context: &Context, template: &str, dst: &str){

    let t = tera.render(template, &context).unwrap();
    let mut f =File::create(&dst)
            .expect(format!("Could not create file: ").as_str());

    f.write_all(t.as_bytes())
        .expect(format!("Could not write to file: ").as_str());

    println!("{} rendered", dst);
}

fn all_render(tera: &Tera, context: &mut Context, template: &str, dst: &str, pic_list: &Vec<PicInfo>){
    //render pages of all
    let header = HeaderConfig("grey".to_string(),"black".to_string(),"grey".to_string());
    context.insert("header",&header);

    let mut cnt = 0;
    let mut pic_vec = Vec::new();

    while cnt < pic_list.len(){
        pic_vec.push(pic_list[cnt].clone());
        cnt = cnt + 1;

        if (cnt+1) % 10 == 0 || cnt == pic_list.len(){
            let page = (cnt / 10) + 1;
            let mut new_dst = String::from(dst);
            context.insert("page", &page);
            context.insert("page_total", &((pic_list.len()-1) / 10 + 1));
            context.insert("items", &pic_vec);
            context.insert("url_prefix", "../");
            //Relative Location is used here. May need some amendments.
            new_dst += &page.to_string();
            new_dst +=  &".html".to_string();
            render(&tera, &context, template, &new_dst);

            while pic_vec.len() > 0 {
                pic_vec.pop();
            }
        }
    }
}

fn get_article_url(name: &str)->String{
    "/articles/".to_string()+&name+".html"
}

fn article_render(tera: &Tera, context: &mut Context, articles: &Vec<ArticleInfo>){
    let header = HeaderConfig("grey".to_string(),"grey".to_string(),"black".to_string());
    context.insert("header",&header);

    /* 
    for article in articles {
        context.insert("body",&article.content);
        render(&tera, &context, "article.html",&("public/articles/".to_string()+&article.name+".html"));
    }*/
    let mut has_prev;
    let mut prev;
    let mut prev_text;
    let mut has_next;
    let mut next;
    let mut next_text;

    for i in 0..articles.len(){
        if i==0 {
            has_prev = false;
            prev = String::new();
            prev_text = String::new();
        }else{
            has_prev = true;
            prev = get_article_url(&articles[i-1].name);
            prev_text = articles[i-1].title.clone();
        }
        if i == articles.len()-1 {
            has_next = false;
            next = String::new();
            next_text = String::new();
        }else{
            has_next = true;
            next = get_article_url(&articles[i+1].name);
            next_text = articles[i+1].title.clone() ;
        }

        let nav = NavConfig{
            has_next,
            has_prev,
            next,
            prev,
            next_text,
            prev_text
        };
        context.insert("body",&articles[i].content);
        context.insert("nav",&nav);
        render(&tera, &context, "article.html",&("public/articles/".to_string()+&articles[i].name+".html"));
    }
}

#[derive(Serialize)]
#[derive(Debug)]
struct ArticleIndexItem{
    name: String,
    date: String,
    url: String,
}

fn article_index_render(tera: &Tera, context: &mut Context, articles: &Vec<ArticleInfo>){
    let mut items = Vec::new();

    let header = HeaderConfig("grey".to_string(),"grey".to_string(),"black".to_string());
    context.insert("header",&header);
    
    for article in articles {
        let name = article.title.clone();
        let url = "/articles/".to_string() + &article.name + ".html";
        let naive_datetime = NaiveDateTime::from_timestamp(article.date, 0);
        let datetime: DateTime<Utc> = DateTime::from_utc(naive_datetime, Utc);
        let item = ArticleIndexItem{
            name,
            date: datetime.to_string(),
            url
        };
        items.push(item);
    }
    context.insert("items",&items);
    render(&tera, &context, "article_index.html","public/article_index.html");
}

pub fn render_main(tera: &Tera, config: &Config, pic_list: &Vec<PicInfo>, articles: &Vec<ArticleInfo>){

    //render main
    let mut context = Context::new();
    
    context.insert("config", &config);
    context.insert("items", &pic_list);
    context.insert("url_prefix", "");

    let mut header = HeaderConfig("black".to_string(),"grey".to_string(),"grey".to_string());
    context.insert("header",&header);
    render(&tera, &context, "index.html", "public/index.html");

    header = HeaderConfig("grey".to_string(),"grey".to_string(),"grey".to_string());
    context.insert("header",&header);
    render(&tera, &context, "about.html", "public/about.html");

    all_render(&tera, &mut context, "all.html", "public/all/", pic_list);
    article_render(&tera, &mut context, &articles);
    article_index_render(&tera, &mut context, &articles);
}