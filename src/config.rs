use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    tab_title: String,
    title: String,
    subtitle: String,
    footer_info: String,
    beian: String,
    pub compress_image: bool,
}

pub fn read() -> Config {
    let yaml_str = std::fs::File::open("./config.yaml")
        .expect("Cannot read config.yaml");
    let config:Config = serde_yaml::from_reader(yaml_str)
        .expect("Failed to parse config.yaml");
    println!("Readed Config. {:?}", config);
    config
}