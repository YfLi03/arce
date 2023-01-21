use std::path::PathBuf;

use rusqlite::params;
use rusqlite::Connection;

use crate::api::err;
use crate::api::folders::{ArticleFolder, ArticleFolderList, PictureFolder, PictureFolderList};

/// getting the article folders that should be monitored
pub fn get_article_folders(conn: &Connection) -> Result<ArticleFolderList, err::Error> {
    let mut stmt = conn.prepare("SELECT * FROM article_folders")?;
    let mut rows = stmt.query(params![])?;
    let mut folders = ArticleFolderList::new();
    while let Some(row) = rows.next()? {
        folders.push(ArticleFolder {
            path: PathBuf::from(row.get::<&str, String>("PATH")?),
            deploy: row.get("DEPLOY")?,
            need_confirm: row.get("CONFIRM")?,
        })
    }
    Ok(folders)
}

/// getting the picture folders that should be monitored
pub fn get_picture_folders(conn: &Connection) -> Result<PictureFolderList, err::Error> {
    let mut stmt = conn.prepare("SELECT * FROM picture_folders")?;
    let mut rows = stmt.query(params![])?;
    let mut folders = PictureFolderList::new();
    while let Some(row) = rows.next()? {
        folders.push(PictureFolder {
            path: PathBuf::from(row.get::<&str, String>("PATH")?),
        })
    }
    Ok(folders)
}

/// adding an article folder that should be monitored
pub fn add_article_folder(conn: &Connection, f: ArticleFolder) -> Result<(), err::Error> {
    let mut stmt = conn.prepare(
        "INSERT INTO article_folders\
        (PATH, CONFIRM, DEPLOY)\
        VALUES (?1, ?2, ?3)",
    )?;
    stmt.execute(params![f.path.to_str(), f.need_confirm, f.deploy])?;
    Ok(())
}

/// adding a picture folder that should be monitored
pub fn add_picture_folder(conn: &Connection, f: PictureFolder) -> Result<(), err::Error> {
    let mut stmt = conn.prepare(
        "INSERT INTO picture_folders\
        (PATH)\
        VALUES (?1)",
    )?;
    stmt.execute(params![f.path.to_str()])?;
    Ok(())
}

/// initializing folders
pub fn init(conn: &Connection) -> Result<(), err::Error> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS article_folders (\
        ID      INTEGER     PRIMARY KEY AUTOINCREMENT,  \
        PATH    TEXT        NOT NULL,\
        DEPLOY  TEXT        NOT NULL,\
        CONFIRM BOOLEAN     NOT NULL\
        )",
        [],
    )?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS picture_folders (\
        ID      INTEGER     PRIMARY KEY AUTOINCREMENT,  \
        PATH    TEXT        NOT NULL\
        )",
        [],
    )?;

    Ok(())
}
