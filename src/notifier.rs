use crate::{api::{folders::{ArticleFolderList, PictureFolderList}, sync::{ConnPool, GlobalConnPool}, err}, model::folders::{get_article_folders, get_picture_folders}};

mod article_folder;
mod picture_folder;

pub fn init() -> Result<(), err::Error>{
    let pool = GlobalConnPool::global().0.clone();
    let a_folders = get_article_folders(&pool.get().unwrap())?;
    let p_folders = get_picture_folders(&pool.get().unwrap())?;
    article_folder::watch_folders(a_folders, pool.clone());
    picture_folder::watch_folders(p_folders, pool.clone());
    Ok(())
}