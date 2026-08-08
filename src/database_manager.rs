use std::ops::ControlFlow::Continue;

use rusqlite::{Connection, Result};

pub fn initialize_database() -> Result<()> {
    let conn = Connection::open("database.db")?;

    conn.execute(
        "CREATE TABLE topics (
                id INTEGER PRIMARY KEY,
                name TEXT NOT NULL,
                lessons INTEGER
            ) STRICT", {})?;
    
    Ok(())
}

pub fn create_topic(name: &str) -> Result<()> {

    let conn = Connection::open("database.db")?;

    let sql = format!("CREATE TABLE \"{}\" (
                     id INTEGER PRIMARY KEY,
                     name TEXT NOT NULL,
                     due INTEGER NOT NULL
                    ) STRICT", name);
    conn.execute(&sql, {})?;

    conn.execute("INSERT INTO topics (name, lessons) VALUES (:name, :lessons)", (name, 0))?;

    Ok(())
}