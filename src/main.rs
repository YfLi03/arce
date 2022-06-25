

mod parser;
mod config;
mod pic_selector;
mod renderer;
mod markdown;
mod init;


fn main() {
    println!("Main Running.");
    init::init_public_folder();
    markdown::render("about.md".to_string(),"template/partial/about_content.html".to_string());
    
    let web = parser::parse();
    
    let config_info =  config::read();
    
    let mut pic_list = pic_selector::read();

    let mut pic_vec = Vec::new();
    while !pic_list.is_empty() {
        pic_vec.push(pic_list.pop().unwrap());
        //pic_list.pop();
    }
    
    renderer::render_main(&web, &config_info, &pic_vec);

    println!("Main completed");

    println!("Press any key and Enter to continue...");
    let mut temp = String::new();
	std::io::stdin().read_line(&mut temp).expect("Failed to read line");
}
