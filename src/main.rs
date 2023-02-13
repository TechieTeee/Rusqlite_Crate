use std::error::Error;
use rusqlite::{params, Connection};

fn main() -> Result<(), Box<dyn Error>> {
    let conn = Connection::open_in_memory()?;

    conn.execute(
        "CREATE TABLE users (
                  id              INTEGER PRIMARY KEY,
                  name            TEXT NOT NULL,
                  email           TEXT NOT NULL
                  )",
        params![],
    )?;

    let name = "John Doe".to_string();
    let email = "jdoe@example.com".to_string();
    conn.execute(
        "INSERT INTO users (name, email)
                  VALUES (?1, ?2)",
        params![name, email],
    )?;

    let mut stmt = conn.prepare("SELECT id, name, email FROM users")?;
    let users = stmt.query_map(params![], |row| {
        Ok((row.get(0)?, row.get(1)?, row.get(2)?))
    })?;

    for user in users {
        let (id, name, email) = user?;
        println!("id: {} name: {} email: {}", id, name, email);
    }

    Ok(())
}

