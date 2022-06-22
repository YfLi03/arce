

mod parser;
mod config;
mod pic_selector;
mod renderer;


fn main() {
    println!("Main Running.");
    
    let web = parser::parse();
    
    let config_info =  config::read();
    
    let mut pic_list = pic_selector::read();

    let mut pic_vec = Vec::new();
    while !pic_list.is_empty() {
        pic_vec.push(pic_list.pop().unwrap());
        //pic_list.pop();
    }
    
    renderer::render(&web, &config_info, &pic_vec);

    println!("Main completed");
}
