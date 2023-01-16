use std::{fs::{create_dir_all, copy}, process::Command, thread::{self, sleep}, time::Duration};
use log::{warn, info};

use crate::api::{err, config::{GlobalConfig, self, CONFIG}, articles::Article};

const FOLDERS: [&str; 6] = 
    ["public/index", "public/gallery",
    "public/css", "public/picture",
    "public/category", "public/article"];


fn init() -> Result<(), err::Error>{
    for folder in FOLDERS {
        create_dir_all(folder)?;
    }
    copy("css/main.css", "public/css/main.css")?;
    Ok(())
}

fn markdown_paser() -> Result<Article, err::Error> {

}

fn render() -> Result<(), err::Error> {

}

fn deploy(){
    let config = GlobalConfig::global();
    let dst = config.scp_server.clone()
                        +":"
                        + &config.scp_web_path;
    match Command::new("scp").arg("-r").arg("public/").arg(&dst).output() {
        Err(e) => {
            warn!("DEPLOY FAILED due to {:?}", e);
        },
        _ => {}
    }
}

pub fn start() {
    init().expect("Error initializing publisher");
    let config = GlobalConfig::global();
    if !config.deploy_auto {
        return;
    }
    thread::spawn(
        ||
        {
            loop {
                sleep(Duration::new(config.deploy_interval.unwrap(), 0));
                info!("Start Publishing");
                if let Err(e) = render() {
                    warn!("Error rendering , {:?}", e);
                    continue;
                }
                deploy();
            }
        }
    );
}