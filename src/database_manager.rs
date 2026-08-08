use std::ops::ControlFlow::Continue;

use rusqlite::{Connection, Result};
use crate::models::Topic;

pub fn initialize_database() -> Result<()> {
    let conn = Connection::open("database.db")?;

    conn.execute(
        "CREATE TABLE topics (
                id INTEGER PRIMARY KEY,
                name TEXT NOT NULL,
                lessons INTEGER NOT NULL,
                due INTEGER NOT NULL
            ) STRICT", {})?;
    
    Ok(())
}

pub fn create_topic(name: &str) -> Result<()> {

    let conn = Connection::open("database.db")?;

    let sql = format!("CREATE TABLE \"{}\" (
                     id INTEGER PRIMARY KEY,
                     name TEXT NOT NULL,
                     next_review INTEGER NOT NULL
                    ) STRICT", name);
    conn.execute(&sql, {})?;

    conn.execute("INSERT INTO topics (name, lessons, due) VALUES (:name, :lessons, :due)", (name, 0, 0))?;

    Ok(())
}

pub fn load_topics() -> Result<Vec<Topic>> {
    let conn = Connection::open("database.db")?;

    let mut statement = conn.prepare("SELECT name, lessons, due FROM topics")?;

    let rows = statement.query_map([], |row| {
        Ok(Topic {
            title: row.get(0)?,
            lessons: row.get(1)?,
            due: row.get(2)?,
        })
    })?;

    let topics = rows.collect::<Result<Vec<Topic>>>()?;

    Ok(topics)
}