use std::path::PathBuf;

use rusqlite::params;
use rusqlite::Connection;

use crate::api::articles::{ArticleInfo, ArticleList};
use crate::api::err;

/// getting the list of articles stored in the database
pub fn get_articles(conn: &Connection) -> Result<ArticleList, err::Error> {
    let mut stmt = conn.prepare("SELECT * FROM articles")?;
    let mut rows = stmt.query(params![])?;

    let mut articles = ArticleList::new();
    while let Some(row) = rows.next()? {
        articles.push(ArticleInfo {
            path: PathBuf::from(row.get::<&str, String>("PATH")?),
            deploy_folder: row.get("DEPLOY_FOLDER")?,
            time: row.get("TIME")?,
        })
    }

    Ok(articles)
}

/// updating an existing article, or creating one
/// two articles are considered different if they have uniqe PATH AND DEPLY_FOLDER
/// ——maybe path itself is enough
pub fn update_article(conn: &Connection, article: ArticleInfo) -> Result<(), err::Error> {
    let mut stmt = conn.prepare(
        "INSERT or REPLACE INTO articles\
        (PATH, DEPLOY_FOLDER, TIME)\
        VALUES (?1, ?2, ?3)\
        ",
    )?;

    stmt.execute(params![
        article.path.to_str(),
        article.deploy_folder,
        article.time
    ])?;

    Ok(())
}

/// deleting an existing article
pub fn delete_article(conn: &Connection, p: PathBuf) -> Result<(), err::Error> {
    let mut stmt = conn.prepare(
        "DELETE FROM articles \
        WHERE PATH = ?1 ",
    )?;

    stmt.execute(params![p.to_str()])?;

    Ok(())
}

/// initializing the articles table
pub fn init(conn: &Connection) -> Result<(), err::Error> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS articles (\
        PATH            TEXT        NOT NULL,\
        DEPLOY_FOLDER   BOOLEAN     NOT NULL,\
        TIME            INTEGER     NOT NULL,\
        PRIMARY KEY(PATH, DEPLOY_FOLDER)\
        )",
        [],
    )?;

    // in current version, initializing means scanning all the monitored folders
    conn.execute("DELETE FROM articles", [])?;

    Ok(())
}
