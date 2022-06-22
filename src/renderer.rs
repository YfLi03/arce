use tera::Tera;
use tera::Context;
use std::fs::{File};
use std::io::Write;

use crate::config::Config;
use crate::pic_selector::PicInfo;

fn render(tera: &Tera, context: &Context, template: String, dst: String){
    let t = tera.render(&template, &context).unwrap();
    let mut f =File::create(dst)
            .expect(format!("Could not create file: ").as_str());

    f.write_all(t.as_bytes())
        .expect(format!("Could not write to file: ").as_str());

    println!("{} rendered", template);
}

pub fn render_main(tera: &Tera, config: &Config, pic_list: &Vec<PicInfo>){

    //render main
    let mut context = Context::new();
    context.insert("config", &config);
    context.insert("items", &pic_list);

    render(&tera, &context, "index.html".to_string(), "public/index.html".to_string());
    render(&tera, &context, "about.html".to_string(), "public/about.html".to_string());


}