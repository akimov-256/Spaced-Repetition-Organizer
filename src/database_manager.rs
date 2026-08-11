use std::fmt::format;

use chrono::{DateTime, Datelike, Duration, Utc, format::Numeric::Timestamp};
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
                interval INTEGER NOT NULL DEFAULT 1,

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

    let next_review = Utc::now() + Duration::days(1);

    let sql = format!("INSERT INTO lessons (name, topic, next_review) VALUES (\"{}\", \"{}\", \"{}\")", lesson, topic, next_review.timestamp());

    conn.execute(&sql, {})?;

    Ok(())
}

pub fn load_lessons(topic: String) -> Result<Vec<Lesson>> {
    let conn = Connection::open("database.db")?;

    let sql = format!("SELECT name, stage, previous_review, next_review FROM lessons WHERE topic == \"{}\"", topic);

    let mut statement = conn.prepare(&sql)?;

    let rows = statement.query_map([], |row| {
        let previous_review: Option<i64> = row.get(2)?;

        let previous_formatted = match previous_review {
            Some(timestamp) => {
                DateTime::from_timestamp(timestamp, 0)
                    .map(|date| date.format("%d-%m-%Y").to_string())
                    .unwrap_or_else(|| "Invalid date".to_string())
            }
            None => "Never".to_string(),
        };

        let next_review = DateTime::from_timestamp(row.get(3)?, 0);

        let next_formatted = next_review.unwrap()
            .format("%d-%m-%Y").to_string();

        let due = next_review.unwrap().signed_duration_since(Utc::now()).num_days();

        Ok(
            Lesson {
                name: row.get(0)?,

                stage: row.get(1)?,
                previous_review: previous_formatted,
                next_review: next_formatted,
                due: due.to_string()
            }
        )
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

    // Get the old interval from the database
    let sql = format!("SELECT interval FROM lessons WHERE topic == \"{}\" AND name == \"{}\"", topic, lesson);

    let interval: i32 = conn.query_row(
        &sql,
        [],
        |row| row.get(0),
    )?;

    // Calculate new interval
    let new_interval: i64 = (interval as f32 * 1.5) as i64;

    let next_review = Utc::now() + Duration::days(new_interval);

    let now = Utc::now().timestamp();

    conn.execute("UPDATE lessons SET stage = ?1, interval = ?2, previous_review = ?3, next_review = ?4 WHERE name == ?5 AND topic == ?6", (stage + 1, new_interval, now, next_review.timestamp(), lesson, topic))?;

    Ok(())
}