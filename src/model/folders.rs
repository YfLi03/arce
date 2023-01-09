use std::path::PathBuf;

use rusqlite::Connection;
use rusqlite::params;

use crate::api::err;
use crate::api::folders::{ArticleFolder, PictureFolder};

type ArticleFolderList = Vec<ArticleFolder>;
type PictureFolderList = Vec<PictureFolder>;

pub fn get_article_folders (conn: &Connection) -> Result<ArticleFolderList, err::Error>{
    let mut stmt = conn.prepare("SELECT * FROM article_folers")?;
    let mut rows = stmt.query(params![])?;
    let mut folders = ArticleFolderList::new();
    while let Some(row) = rows.next()? {
        folders.push(ArticleFolder{
            path: PathBuf::from(row.get::<&str, String>("path")?),
            need_confirm: row.get("confirm")?
        })
    };
    Ok(folders)
}

pub fn get_picture_folders (conn: &Connection) -> Result<PictureFolderList, err::Error>{
    let mut stmt = conn.prepare("SELECT * FROM picture_folers")?;
    let mut rows = stmt.query(params![])?;
    let mut folders = PictureFolderList::new();
    while let Some(row) = rows.next()? {
        folders.push(PictureFolder{
            path: PathBuf::from(row.get::<&str, String>("path")?),
        })
    };
    Ok(folders)
}