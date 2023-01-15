use std::{collections::HashSet, path::PathBuf};

use api::{config::{CONFIG, GlobalConfig}, sync::{ConnPool, GlobalConnPool, CONN_POOL, NeedPublish, NEED_PUBLISH}};
use r2d2_sqlite::SqliteConnectionManager;
/*
mod parser;
mod config;
mod pic_selector;
mod renderer;
mod markdown;
mod init;
mod article;
mod sql;
*/
mod api;
mod model;
mod notifier;

fn init(f: PathBuf){
    let manager = SqliteConnectionManager::file("arce.db");
    let global_conn_pool = GlobalConnPool(r2d2::Pool::new(manager).unwrap());
    CONN_POOL.set(global_conn_pool).unwrap();

    let need_publish = NeedPublish::new(false);
    NEED_PUBLISH.set(need_publish).unwrap();

    let config = GlobalConfig::from_file(f).expect("Reading Config file failed");
    CONFIG.set(config).unwrap();

    let conn = GlobalConnPool::global().0.get().unwrap();
    crate::model::init().expect("Initialize DB failed");
    // read config

    crate::notifier::init().expect("Initialize Article Notifier failed");


    // init renderer
}

fn main() {
    init();
    // CONFIG.set().unwrap();
    /*
    println!("Main Running.");

    init::init_public_folder().expect("Error initializing the folders");

    let web = parser::parse().expect("Error loading templates for tera");

    let config_info =  config::read().expect("Error reading the config");

    //Render the articles and also get their names
    let mut name_set: HashSet<String> = HashSet::new();
    let articles = article::read(&mut name_set);

    let pic_list = pic_selector::read(&config_info,&name_set).unwrap();

    renderer::render_main(&web, &config_info, &pic_list, &articles);
    println!("Main completed.");

    println!("Press any key and Enter to continue...");
    let mut temp = String::new();
    std::io::stdin().read_line(&mut temp).expect("Failed to read line");
    */
}
