

mod parser;
mod config;
mod pic_selector;

fn main() {
    println!("Main Running.");
    
    let web = parser::parse();
    
    let config_info =  config::read();
    
    let pic_list = pic_selector::read();
    /*
    let renderer =  render(&web, &config_info, &pic_list);
    renderer.renderMain();
    renderer.renderAll();
    renderer.renderAbout();
    */
    println!("Main completed");
}
