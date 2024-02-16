use rusqlite::{Connection, Error};

use crate::models::MyDialogue;

pub fn create_db() {
    let conn = Connection::open("hh.db").expect("Failed to establish connection to db");

    conn.execute(
        "CREATE TABLE IF NOT EXISTS users (
            id INTEGER PRIMARY KEY AUTOINCREMENT
            )",
        [],
    )
    .expect("Failed to create users table");

    conn.execute(
        "CREATE TABLE IF NOT EXISTS category (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL
            )",
        [],
    )
    .expect("Failed to create category table");

    conn.execute(
        "CREATE TABLE IF NOT EXISTS homework (
            name TEXT PRIMARY KEY,
            desc TEXT NOT NULL,
            deadline DATE,
            date_created TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
            category_id INTEGER,
            user_id INTEGER,
            FOREIGN KEY (category_id) REFERENCES category(id),
            FOREIGN KEY (user_id) REFERENCES users(id)
            )",
        [],
    )
    .expect("Failed to create homework_hub table");

<<<<<<< HEAD
    log::info!("created database successfully!");
}

pub fn select_all_categories() -> Result<Vec<String>, Error> {
    let conn = Connection::open("your_database.db")?;

    let mut stmt = conn.prepare("SELECT name FROM category")?;

    let category_iter = stmt.query_map([], |row| {
        Ok(row.get(0)?)
    })?;

    let mut categories = Vec::new();

    for name in category_iter {
        categories.push(name?);
    }

    Ok(categories)
}

pub fn insert_user(msg: MyDialogue) -> Result<(), Error> {
    let conn = Connection::open("your_database.db")?;

    let mut stmt = conn.prepare("SELECT COUNT(*) FROM users WHERE id = ?")?;
    let user_exists: i64 = stmt.query_row([msg.chat_id().to_string()], |row| row.get(0))?;

    if user_exists == 0 {
        conn.execute("INSERT INTO users(id) VALUES (?)", [&msg.chat_id().to_string()])?;
    } else {

    }

    Ok(())
}

pub fn insert_category(name: &str) -> Result<(), Error> {
    let conn = Connection::open("your_database.db")?;

    conn.execute(
        "INSERT INTO category (name) VALUES (?)",
        &[name],
    )?;

    Ok(())
}

pub fn select_categorie(name: &str) -> Result<i32, Error> {
    let conn = Connection::open("your_database.db")?;

    let category_id: i32 = conn.query_row(
        "SELECT id FROM category WHERE name = ?",
        &[name],
        |row| row.get(0)
    )?;

    Ok(category_id)
}

pub fn insert_homework(name: &str, desc: &str, deadline: &str, category_id: &i32, msg: MyDialogue) -> Result<(), Error> {
    let conn = Connection::open("your_database.db")?;

    conn.execute(
        "INSERT INTO homework_hub (name, desc, deadline, category_id, user_id) VALUES (?, ?, ?, ?, ?)",
        &[name, desc, deadline, &category_id.to_string(), &msg.chat_id().to_string()],
    )?;

    Ok(())
=======
    log::info!("DB created successfully!");
>>>>>>> 37cdce1 (chore: format code)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_db_and_insert() {
        create_db();
        let conn = Connection::open("hh.db").expect("Failed to open database");
        conn.execute("INSERT INTO users DEFAULT VALUES", [])
            .expect("Failed to insert into users");
        conn.execute("INSERT INTO category (name) VALUES (?)", ["test"])
            .expect("Failed to insert into category");
        conn.execute(
            "INSERT INTO homework_hub (name, desc, deadline, category_id, user_id) VALUES (?, ?, ?, ?, ?)",
            ["test", "test", "2021-12-12", "1", "1"],
        )
        .expect("Failed to insert into hh.db");
    }
}
