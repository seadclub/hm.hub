use crate::models::MyDialogue;
use rusqlite::{Connection, Error};

fn create_connection() -> Result<Connection, Error> {
    Connection::open("hh.db")
}

pub fn create_db() {
    let conn = create_connection().expect("Failed to open database connection at create_db");

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
    .expect("Failed to create `homework` table");

    log::info!("Database created successfully!");
}

pub fn select_all_categories() -> Result<Vec<String>, Error> {
    let conn =
        create_connection().expect("Failed to open database connection at select_all_categories");

    let mut stmt = conn.prepare("SELECT name FROM category")?;

    let category_iter = stmt.query_map([], |row| Ok(row.get(0)?))?;

    let mut categories = Vec::new();

    for name in category_iter {
        categories.push(name?);
    }

    Ok(categories)
}

pub fn insert_user(msg: MyDialogue) -> Result<(), Error> {
    let conn = create_connection().expect("Failed to open database connection at insert_user");

    let mut stmt = conn.prepare("SELECT COUNT(*) FROM users WHERE id = ?")?;
    let user_exists: i64 = stmt.query_row([msg.chat_id().to_string()], |row| row.get(0))?;

    if user_exists == 0 {
        conn.execute(
            "INSERT INTO users(id) VALUES (?)",
            [&msg.chat_id().to_string()],
        )?;
    }

    Ok(())
}

pub fn insert_category(name: &str) -> Result<(), Error> {
    let conn = create_connection().expect("Failed to open database connection at insert_category");

    conn.execute("INSERT INTO category (name) VALUES (?)", &[name])?;

    Ok(())
}

pub fn select_category(name: &str) -> Result<i32, Error> {
    let conn = create_connection().expect("Failed to open database connection at select_category");

    let category_id: i32 =
        conn.query_row("SELECT id FROM category WHERE name = ?", &[name], |row| {
            row.get(0)
        })?;

    Ok(category_id)
}

pub fn insert_homework(
    name: &str,
    desc: &str,
    deadline: &str,
    category_id: &i32,
    msg: MyDialogue,
) -> Result<(), Error> {
    let conn = create_connection().expect("Failed to open database connection at insert_homework");

    conn.execute(
        "INSERT INTO homework (name, desc, deadline, category_id, user_id) VALUES (?, ?, ?, ?, ?)",
        &[
            name,
            desc,
            deadline,
            &category_id.to_string(),
            &msg.chat_id().to_string(),
        ],
    )?;

    Ok(())
}
