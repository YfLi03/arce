
use rusqlite::{Connection, Result};
use crate::pic_selector::PicInfo;

pub enum ConnectMode{
    Pics,
    Articles,
    None
}

///connect to db
pub fn connect(dst:&str, c: &ConnectMode) -> Result<Connection> {
    let conn = Connection::open(dst)?;
    if let Pics = c {
        conn.execute(
            "create table if not exists pic_list (
                id      INTEGER     PRIMARY KEY AUTOINCREMENT,
                url     TEXT        NOT NULL,
                name   TEXT        NOT NULL,
                size    INTEGER     NOT NULL,
                date    TEXT,
                parameters  TEXT,
                camera      TEXT,
                selected    BOOLEAN,
                class       TEXT,
                has_link    BOOLEAN,
                link        TEXT
            )",[]).unwrap();
        }

    if let Articles = c {
        conn.execute(
            "create table if not exists articles (
                id      INTEGER     PRIMARY KEY AUTOINCREMENT,
                title     TEXT        NOT NULL,
                name   TEXT        NOT NULL,    
                content    INTEGER     NOT NULL
            )",[]).unwrap();
    }    
    Ok(conn)
}


///insert pic
pub fn insertp(conn: &Connection, pic: &PicInfo) -> Result<()>{
    let mut stmt =  conn.prepare("INSERT INTO pic_list (url, name, size, date, parameters, camera, selected, class, has_link, link)\
    VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)")?;
    stmt.execute(pic.get_sql())?;
    Ok(())
}

///TO QUERY WHETHER A PIC EXISTED IN THE DATABASE
///FIRST, JUDGE BY url
///     - MULTIPLE DATA, JUDGE BY SIZE, SELECT THE FIRST ONE
///     - ONE DATA, GET IT
///IF NO DATA WITH THE SAME NAME,
/// SEARCH WITH PIC_SIZE, PRINT THE INFO
pub fn queryp(conn: &Connection, url: &str, size: u64) -> Result<Option<PicInfo>>{
//    let mut stmt = conn.prepare("SELECT id, name, time_created, data FROM person").unwrap();
    let mut stmt = conn.prepare("SELECT * FROM pic_list WHERE url = ?1 ORDER BY id DESC")?;
    let mut rows = stmt.query(&[&url])?;
    while let Some(row) = rows.next()? {
        let t: u64 = row.get(3)?;
        let selected: bool = row.get(7)?;
        let has_link: bool =  row.get(9)?;
        if t == size {
            let item: PicInfo = PicInfo::new(
                row.get(1)?,
                row.get(2)?,
                row.get(3)?,
                row.get(4)?,
                row.get(5)?,
                row.get(6)?,
                selected,
                row.get(8)?,
                has_link,
                row.get(10)?
            );
            return Ok(Some(item));
        }
    };

    let mut stmt = conn.prepare("SELECT * FROM pic_list WHERE size = ?1 ORDER BY id DESC")?;
    let mut rows = stmt.query(&[&size])?;
    while let Some(row) = rows.next()? {
        let item: PicInfo = PicInfo::new(
            url.to_string(),
            row.get(2)?,
            row.get(3)?,
            row.get(4)?,
            row.get(5)?,
            row.get(6)?,
            row.get(7)?,
            row.get(8)?,
            row.get(9)?,
            row.get(10)?
        );
        let new_url:String = row.get(1)?;
        println!("Using pic {} as pic {}, since they have the same size.", &new_url, &url);
        return Ok(Some(item));
    }
    return Ok(None);
}

