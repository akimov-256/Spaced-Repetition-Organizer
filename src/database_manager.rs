use chrono::Utc;
use rusqlite::{Connection, Result};
use crate::models::{Lesson, Topic};

pub fn initialize_database() -> Result<()> {
    let conn = Connection::open("database.db")?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS topics (
                id INTEGER PRIMARY KEY,
                name TEXT NOT NULL,
                lessons INTEGER NOT NULL,
                due INTEGER NOT NULL
            ) STRICT", {})?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS lessons (
                id INTEGER PRIMARY KEY,
                name TEXT NOT NULL,
                topic TEXT NOT NULL,
                
                stage INTEGER NOT NULL DEFAULT 1,
                previous_review INTEGER,
                next_review INTEGER NOT NULL
            ) STRICT", {})?;
    
    Ok(())
}

pub fn create_topic(name: &str) -> Result<()> {

    let conn = Connection::open("database.db")?;

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

pub fn delete_topic(name: String) -> Result<()> {
    let conn = Connection::open("database.db")?;

    let sql = format!("DELETE FROM topics WHERE name == \"{}\"", name);

    conn.execute(&sql, {})?;

    Ok(())
}

pub fn add_lesson(topic: String, lesson: String) -> Result<()> {
    let conn = Connection::open("database.db")?;

    let sql = format!("INSERT INTO lessons (name, topic, next_review) VALUES (\"{}\", \"{}\", \"{}\")", lesson, topic, 0);

    conn.execute(&sql, {})?;

    Ok(())
}

pub fn load_lessons(topic: String) -> Result<Vec<Lesson>> {
    let conn = Connection::open("database.db")?;

    let sql = format!("SELECT name, stage, previous_review, next_review FROM lessons WHERE topic == \"{}\"", topic);

    let mut statement = conn.prepare(&sql)?;

    let rows = statement.query_map([], |row| {
        Ok(Lesson {
            name: row.get(0)?,

            stage: row.get(1)?,
            previous_review: row.get(1)?,
            next_review: row.get(1)?
        })
    })?;

    let lessons = rows.collect::<Result<Vec<Lesson>>>()?;

    Ok(lessons)
}

pub fn delete_lesson(topic: String, lesson:String) -> Result<()> {
    let conn = Connection::open("database.db")?;

    let sql = format!("DELETE FROM lessons WHERE topic == \"{}\" AND name == \"{}\"", topic, lesson);

    conn.execute(&sql, [])?;

    Ok(())
}

pub fn review_lesson(topic: String, lesson: String, stage: i32) -> Result<()> {
    let conn = Connection::open("database.db")?;

    let now = Utc::now().timestamp();

    conn.execute("UPDATE lessons SET stage = ?1, previous_review = ?2, next_review = ?3 WHERE name == ?4 AND topic == ?5", (stage + 1, now, 123456, lesson, topic))?;

    Ok(())
}