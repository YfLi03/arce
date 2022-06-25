use tera::Tera;
use tera::Context;
use std::fs::{File};
use std::io::Write;

use crate::config::Config;
use crate::pic_selector::PicInfo;

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
            new_dst += &page.to_string();
            new_dst +=  &".html".to_string();
            render(&tera, &context, template, &new_dst);

            while pic_vec.len() > 0 {
                pic_vec.pop();
            }
        }
    }
}


pub fn render_main(tera: &Tera, config: &Config, pic_list: &Vec<PicInfo>){

    //render main
    let mut context = Context::new();
    context.insert("config", &config);
    context.insert("items", &pic_list);


    render(&tera, &context, "index.html", "public/index.html");
    render(&tera, &context, "about.html", "public/about.html");
    all_render(&tera, &mut context, "all.html", "public/all/", pic_list);


}