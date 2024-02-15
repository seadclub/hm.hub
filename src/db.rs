use rusqlite::Connection;

pub fn create_db() {
    let conn = Connection::open("your_database.db").expect("Failed to open database");

    conn.execute(
        "CREATE TABLE IF NOT EXISTS users (
             id INTEGER PRIMARY KEY AUTOINCREMENT
         )",
        [],
    ).expect("Failed to create users table");

    conn.execute(
        "CREATE TABLE IF NOT EXISTS category (
             id INTEGER PRIMARY KEY AUTOINCREMENT,
             name TEXT NOT NULL
         )",
        [],
    ).expect("Failed to create category table");

    conn.execute(
        "CREATE TABLE IF NOT EXISTS homework_hub (
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
    ).expect("Failed to create homework_hub table");

    log::info!("created database successfully!");
}