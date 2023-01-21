/*
 * Monitors of the folders
 */
use crate::{
    api::{err, sync::GlobalConnPool},
    model::folders::{get_article_folders, get_picture_folders},
};

mod article_folder;
mod picture_folder;

/// initializing notifier for article and picture folders
pub fn init() -> Result<(), err::Error> {
    let pool = GlobalConnPool::global().0.clone();
    let a_folders = get_article_folders(&pool.get().unwrap())?;
    let p_folders = get_picture_folders(&pool.get().unwrap())?;
    article_folder::watch_folders(a_folders);
    picture_folder::watch_folders(p_folders);
    Ok(())
}
