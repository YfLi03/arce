/// initializing folders
use std::fs::{create_dir_all, copy};
use std::error::Error;

const FOLDERS: [&str; 7] = 
    ["public/all", "public/articles",
    "template/temp", "source/article",
    "public/css", "public/gallery/all",
    "public/gallery/selected"];

pub fn init_public_folder() -> Result<(), Box<dyn Error>>{
    for folder in FOLDERS {
        create_dir_all(folder)?;
    }
    copy("css/main.css", "public/css/main.css")?;
    println!("Initted folders.");
    Ok(())
}