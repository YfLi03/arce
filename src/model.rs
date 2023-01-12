use rusqlite::Connection;

use crate::api::err;

pub mod articles;
pub mod folders;
pub mod pictures;

pub fn init(conn: &Connection) -> Result<(), err::Error> {
    folders::init(conn)?;
    articles::init(conn)?;
    pictures::init(conn)?;
    Ok(())
}
