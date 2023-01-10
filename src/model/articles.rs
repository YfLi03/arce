use std::path::PathBuf;

use rusqlite::Connection;
use rusqlite::params;

use crate::api::err;
use crate::api::articles::{ArticleInfo, ArticleList};


pub fn get_articles(conn: &Connection) -> Result<ArticleList, err::Error>{
    let mut stmt = conn.prepare("SELECT * FROM articles")?;
    let mut rows = stmt.query(params![])?;
    let mut articles = ArticleList::new();
    while let Some(row) = rows.next()? {
        articles.push(ArticleInfo{
            path: PathBuf::from(row.get::<&str, String>("PATH")?),
            deploy_path: row.get("DEPLOY_PATH")?,
            time: row.get("TIME")?
        })
    };
    Ok(articles)
}

pub fn update_article(conn: &Connection, article: ArticleInfo) -> Result<(), err::Error> {
    let mut stmt = conn.prepare("INSERT or REPLACE INTO articles\
        (PATH, DEPLOY_PATH, TIME)\
        VALUES (?1, ?2, ?3)\
        ")?;
    stmt.execute(params![article.path.to_str(), article.deploy_path, article.time])?;
    Ok(())
}

pub fn delete_article(conn: &Connection, p: PathBuf) -> Result<(), err::Error> {
    let mut stmt = conn.prepare("DELETE FROM articles\
        WHERE PATH = ?1 ")?;
    stmt.execute(params![p.to_str()])?;
    Ok(())
}

pub fn init (conn: &Connection) -> Result<(), err::Error> {
    conn.execute("CREATE TABLE IF NOT EXISTS articles (\
        ID              INTEGER     PRIMARY KEY AUTOINCREMENT,  \
        PATH            TEXT        NOT NULL,\
        DEPLOY_PATH     BOOLEAN     NOT NULL,\
        TIME            INTEGER     NOT NULL\
        ", [])?;

    Ok(())
}