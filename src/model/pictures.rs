use std::path::PathBuf;

use rusqlite::params;
use rusqlite::Connection;

use crate::api::err;
use crate::api::pictures::PPictureList;
use crate::api::pictures::PhotographyPicture;
use crate::api::pictures::Picture;

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

pub fn insert_picture(conn: &Connection, p: Picture) -> Result<PathBuf, err::Error> {
    if let Some(path) = find_picture(conn, &p)? {
        return Ok(path);
    }

    let mut stmt = conn.prepare(
        "INSERT INTO pictures\
    (PATH, HASH, HASH_OLD)\
    VALUES (?1, ?2, ?3)\
    ",
    )?;
    stmt.execute(params![p.path.to_str(), p.hash, p.hash_old])?;
    Ok(p.path)
}

pub fn insert_photography_picture(
    conn: &Connection,
    p: PhotographyPicture,
) -> Result<PathBuf, err::Error> {
    let mut stmt = conn.prepare(
        "SELECT * FROM pictures WHERE\
    HASH = ?1 AND PHOTOGRAPHY = true",
    )?;
    let mut rows = stmt.query(params![p.hash])?;
    if let Some(row) = rows.next()? {
        return Ok(PathBuf::from(row.get::<&str, String>("PATH")?));
    };

    if let Some(ref hash) = p.hash_old {
        let mut stmt = conn.prepare(
            "SELECT * FROM pictures WHERE\
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
    VALUES (?1, ?2, true, ?3, ?4. ?5, ?6, ?7, ?8, ?9, ?10)\
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
    Ok(p.path)
}

pub fn get_photography_pictures(conn: &Connection) -> Result<PPictureList, err::Error> {
    let mut stmt = conn.prepare("SELECT * FROM pictures WHERE PHOTOGRAPHY = true")?;
    let mut rows = stmt.query([])?;
    let mut pictures = PPictureList::new();
    while let Some(row) = rows.next()? {
        // TODO: Need Checking Here
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
            article_link: row.get("ARTICLE_LINK")?,
        })
    }
    Ok(pictures)
}

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
        ",
        [],
    )?;
    Ok(())
}
