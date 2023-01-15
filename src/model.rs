use rusqlite::Connection;

use crate::api::{err, sync::GlobalConnPool};

pub mod articles;
pub mod folders;
pub mod pictures;

pub fn init() -> Result<(), err::Error> {
    let conn = GlobalConnPool::global().0.get().unwrap();
    folders::init(&conn)?;
    articles::init(&conn)?;
    pictures::init(&conn)?;
    Ok(())
}
