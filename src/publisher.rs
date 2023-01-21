mod deployer;
mod markdown;
mod renderer;

use log::{info, warn};
use std::{
    fs::{copy, create_dir_all},
    thread::{self, sleep},
    time::Duration,
};

use crate::{
    api::{
        config::GlobalConfig,
        err,
        pictures::PhotographyPictureBrief,
        sync::{GlobalConnPool, NeedPublish},
    },
    model::pictures::get_photography_pictures,
};

const FOLDERS: [&str; 5] = [
    "public/index",
    "public/gallery",
    "public/css",
    "public/picture",
    "public/category",
];

/// initialize the folders
fn init() -> Result<(), err::Error> {
    for folder in FOLDERS {
        create_dir_all(folder)?;
    }

    copy("css/main.css", "public/css/main.css")?;
    copy("css/typora.css", "public/css/typora.css")?;

    let names: Vec<_> = renderer::TERA.get_template_names().collect();
    info!("Parsed {} Templates: {:?}", names.len(), names);

    Ok(())
}

/// start publishing
fn publish() -> Result<(), err::Error> {
    info!("Start publishing");
    let conn = GlobalConnPool::global().0.get().unwrap();

    // get the articles
    let articles = markdown::process_articles()?;

    // get the pictures
    let mut pictures = get_photography_pictures(&conn)?;
    pictures.sort_by(|a, b| (&b).date.cmp(&a.date));
    let pictures: Vec<PhotographyPictureBrief> = pictures
        .into_iter()
        .map(|p| PhotographyPictureBrief::from(p))
        .collect();

    // render the html
    renderer::render(articles, pictures)?;

    // deploy the pages
    deployer::deploy();

    info!("Published");
    Ok(())
}

/// start publisher thread
pub fn start() {
    init().expect("Error initializing publisher");

    let config = GlobalConfig::global();
    if !config.deploy_auto {
        return;
    }

    thread::spawn(|| loop {
        // Timer
        sleep(Duration::new(config.deploy_interval.unwrap(), 0));

        // check whether publish is necessary
        let need_publish = NeedPublish::global().get();
        if !need_publish {
            continue;
        };

        // publish
        if let Err(e) = publish() {
            warn!("Error publishing , {:?}", e);
            continue;
        }

        NeedPublish::global().set(false);
    });
}
