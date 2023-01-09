/*
    Read the site's config info.
*/

use serde::{Serialize, Deserialize};
use std::error::Error;

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    tab_title: String,
    title: String,
    subtitle: String,
    footer_info: String,
    beian: String,
    pub compress_image: bool,
}

pub fn read() -> Result<Config, Box<dyn Error>> {
    let yaml_str = std::fs::File::open("./config.yaml")?;
    let config:Config = serde_yaml::from_reader(yaml_str)?;
    println!("Readed Config. {:?}", config);
    Ok(config)
}