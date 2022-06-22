use tera::Tera;
use tera::Context;
use std::fs::{self, File};
use std::io::Write;

use crate::config::Config;
use crate::pic_selector::PicInfo;

pub fn render(tera: &Tera, config: &Config, pic_list: &Vec<PicInfo>){
    //render main
    let mut context = Context::new();
    context.insert("config", &config);
    context.insert("items", &pic_list);
    let t = tera.render("index.html", &context).unwrap();
    let mut f =File::create("public/index.html")
            .expect(format!("Could not create file: ").as_str());

    f.write_all(t.as_bytes())
        .expect(format!("Could not write to file: ").as_str());

}