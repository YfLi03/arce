use std::collections::HashSet;

mod parser;
mod config;
mod pic_selector;
mod renderer;
mod markdown;
mod init;
mod article;


fn main() {
    println!("Main Running.");
    init::init_public_folder();
    
    let web = parser::parse();
    let config_info =  config::read();

    //Render the articles and also get their names
    let mut name_set: HashSet<String> = HashSet::new();
    let articles = article::read(&mut name_set);

    let pic_list = pic_selector::read(&config_info,&name_set);
    
    renderer::render_main(&web, &config_info, &pic_list, &articles);
    println!("Main completed.");

    println!("Press any key and Enter to continue...");
    let mut temp = String::new();
	std::io::stdin().read_line(&mut temp).expect("Failed to read line");
}
