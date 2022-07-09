/*
    Initting the folders
*/
use std::fs::{create_dir_all, copy};

pub fn init_public_folder(){
    create_dir_all("public/all")
        .expect("Err Initializing folders");
    create_dir_all("public/articles")
        .expect("Err Initializing folders");
    create_dir_all("template/temp")
        .expect("Err Initializing folders");
    create_dir_all("source/article")
        .expect("Err Initializing folders");
    create_dir_all("public/css")
        .expect("Err Initializing folders");
    create_dir_all("public/gallery/all")
        .expect("Err Initializing folders");
    create_dir_all("public/gallery/selected")
        .expect("Err Initializing folders");
    copy("css/main.css", "public/css/main.css")
        .expect("Err Copying main.css");
    println!("Initted folders.")


}