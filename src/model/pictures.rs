use std::path::PathBuf;

use log::debug;
use rusqlite::{params,Connection};

use crate::api::err;
use crate::api::pictures::{PPictureList, PhotographyPicture, Picture};

/// finding a picture in the database. both hash and old hash is used.
/// returns Ok(none) if it's not found
/// returns Some(path) if it exists. the path is the actual path locally
pub fn find_picture(conn: &Connection, p: &Picture) -> Result<Option<PathBuf>, err::Error> {
    let mut stmt = conn.prepare("SELECT * FROM pictures WHERE HASH = ?1")?;
    let mut rows = stmt.query(params![p.hash])?;
    if let Some(row) = rows.next()? {
        return Ok(Some(PathBuf::from(row.get::<&str, String>("PATH")?)));
    };

    if let Some(ref hash) = p.hash_old {
        let mut stmt = conn.prepare("SELECT * FROM pictures WHERE HASH_OLD = ?1")?;
        let mut rows = stmt.query(params![hash])?;
        if let Some(row) = rows.next()? {
            return Ok(Some(PathBuf::from(row.get::<&str, String>("PATH")?)));
        };
    };
    
    Ok(None)
}

/// inserting a picture
/// returns the path of the inserted picture, which may not be that of which you inserted
/// as the picture may already exist in the database
pub fn insert_picture(conn: &Connection, p: &Picture) -> Result<PathBuf, err::Error> {
    // pictures not labeled as photoography can't be compressed
    if let Some(path) = find_picture(conn, &p)? {
        debug!("Pic already exists {:?}", p);
        return Ok(path);
    }

    let mut stmt = conn.prepare(
        "INSERT INTO pictures\
    (PATH, HASH, HASH_OLD)\
    VALUES (?1, ?2, ?3)\
    ",
    )?;
    stmt.execute(params![p.path.to_str(), p.hash, p.hash_old])?;

    Ok(p.path.clone())
}

/// inserting a photography picture
/// returns the path of the inserted picture, which may not be that of which you inserted
/// as the picture may already exist in the database, and is labeled as photography
pub fn insert_photography_picture(
    conn: &Connection,
    p: &mut PhotographyPicture,
) -> Result<PathBuf, err::Error> {

    let mut stmt = conn.prepare(
        "SELECT * FROM pictures WHERE \
    HASH = ?1 AND PHOTOGRAPHY = true",
    )?;
    let mut rows = stmt.query(params![p.hash])?;
    if let Some(row) = rows.next()? {
        return Ok(PathBuf::from(row.get::<&str, String>("PATH")?));
    };

    if let Some(ref hash) = p.hash_old {
        let mut stmt = conn.prepare(
            "SELECT * FROM pictures WHERE \
        HASH_OLD = ?1 AND PHOTOGRAPHY = true",
        )?;
        let mut rows = stmt.query(params![hash])?;
        if let Some(row) = rows.next()? {
            return Ok(PathBuf::from(row.get::<&str, String>("PATH")?));
        };
    }

    let mut stmt = conn.prepare(
        "INSERT INTO pictures\
    (PATH, HASH, PHOTOGRAPHY, HASH_OLD, SELECTED, TITLE, PARAMS, DATE, CAMERA, DIRECTION, ARTICLE)\
    VALUES (?1, ?2, true, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)\
    ",
    )?;
    stmt.execute(params![
        p.path.to_str(),
        p.hash,
        p.hash_old,
        p.selected,
        p.title,
        p.params,
        p.date,
        p.camera,
        p.direction,
        p.article_link
    ])?;

    Ok(p.path.clone())
}

/// getting all the pictures labeled as PHOTOGRAPHY
pub fn get_photography_pictures(conn: &Connection) -> Result<PPictureList, err::Error> {
    let mut stmt = conn.prepare("SELECT * FROM pictures WHERE PHOTOGRAPHY = true")?;
    let mut rows = stmt.query([])?;
    let mut pictures = PPictureList::new();
    while let Some(row) = rows.next()? {
        pictures.push(PhotographyPicture {
            hash_old: row.get("HASH_OLD")?,
            hash: row.get("HASH")?,
            path: PathBuf::from(row.get::<&str, String>("PATH")?),
            selected: row.get("SELECTED")?,
            title: row.get("TITLE")?,
            params: row.get("PARAMS")?,
            date: row.get("DATE")?,
            camera: row.get("CAMERA")?,
            direction: row.get("DIRECTION")?,
            article_link: row.get("ARTICLE")?,
        })
    }
    Ok(pictures)
}

/// initializing pictures table
pub fn init(conn: &Connection) -> Result<(), err::Error> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS pictures (\
        ID              INTEGER     PRIMARY KEY AUTOINCREMENT,  \
        PATH            TEXT        NOT NULL,\
        HASH            TEXT        NOT NULL,\
        PHOTOGRAPHY     BOOLEAN     NOT NULL    DEFAULT false,\
        HASH_OLD        TEXT,\
        SELECTED        BOOLEAN     DEFAULT false,\
        TITLE           TEXT,\
        PARAMS          TEXT,\
        DATE            TEXT,\
        CAMERA          TEXT,\
        DIRECTION       TEXT,\
        ARTICLE         TEXT\
        )",
        [],
    )?;
    Ok(())
}
