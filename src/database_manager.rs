use rusqlite::{Connection, Result};

pub fn initialize_database() -> Result<()> {
    let conn = Connection::open_in_memory()?;

    conn.execute(
        "CREATE TABLE topics (
                id INTEGER PRIMARY KEY,
                name TEXT NOT NULL,
                lessons INTEGER
            ) STRICT", {})?;

    conn.execute("INSERT INTO topics (name, lessons) VALUES (?1, ?2)", ("arabic", 3))?;

    let mut stmt = conn.prepare("SELECT name, lessons FROM topics")?;
        
        let rows = stmt.query_map([], |row| {
        let name: String = row.get(0)?;
        let lessons: i32 = row.get(1)?;
        
        Ok((name, lessons))
    })?;
    
    for row in rows {
        let (name, lessons) = row?;
        println!("{name}: {lessons} lessons");
    }
    
    Ok(())
}