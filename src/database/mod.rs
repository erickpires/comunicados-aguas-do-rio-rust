use chrono::NaiveDate;
use rusqlite::Connection;

use crate::error::Error;

pub struct Database {
    connection: Connection,
}

impl<'a> Database {
    pub fn new() -> Result<Self, Error> {
        let connection = Connection::open("./data.db")?;

        connection.execute("CREATE TABLE IF NOT EXISTS Posts (
            id    TEXT PRIMARY KEY,
            date  DATETIME,
            handledAt  DATETIME
        )", ())?;

        Ok(Self {
            connection
        })
    }

    pub fn post_exists(&self, id: &str) -> Result<bool, Error> {
        let mut stmt = self.connection.prepare("SELECT id FROM Posts WHERE id = ?1")?;
        let mut rows = stmt.query([id])?;

        rows.next().map(|r| r.is_some()).map_err(|e| e.into())
    }

    pub fn save_post(&self, id: &str, date: &Option<NaiveDate>) -> Result<(), Error>{
        let date_str = date.map(|d| d.format("%Y-%m-%d").to_string()).unwrap_or("NULL".to_string());

        let mut stmt = self.connection.prepare("INSERT INTO Posts (id, date, handledAt) VALUES (?1, ?2, datetime('now'))")?;
        stmt.execute(rusqlite::params![id, &date_str])?;

        Ok(())
    }
}