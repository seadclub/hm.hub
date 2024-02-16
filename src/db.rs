use rusqlite::Connection;

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

    log::info!("DB created successfully!");
}
