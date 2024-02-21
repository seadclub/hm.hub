use rusqlite::{Connection, Error};

fn create_connection() -> Result<Connection, Error> {
    Connection::open("hh.db")
}

pub fn create_db() -> Result<(), Error> {
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

    Ok(())
}

pub fn select_all_categories() -> Result<Vec<String>, Error> {
    let conn =
        create_connection().expect("Failed to open database connection at select_all_categories");

    let mut stmt = conn.prepare("SELECT name FROM category")?;

    let categories = stmt
        .query_map([], |row| row.get(0))?
        .map(|result| result.unwrap())
        .collect();

    Ok(categories)
}

pub fn insert_user(user_id: &str) -> Result<(), Error> {
    let conn =
        create_connection().expect("Failed to open database connection at insert_user_by_id");

    let mut stmt = conn.prepare("SELECT COUNT(*) FROM users WHERE id = ?")?;
    let user_exists: i64 = stmt.query_row([user_id], |row| row.get(0))?;

    if user_exists == 0 {
        conn.execute("INSERT INTO users(id) VALUES (?)", [user_id])?;
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
    user_id: &str,
) -> Result<(), Error> {
    let conn = create_connection().expect("Failed to open database connection at insert_homework");

    conn.execute(
        "INSERT INTO homework (name, desc, deadline, category_id, user_id) VALUES (?, ?, ?, ?, ?)",
        &[name, desc, deadline, &category_id.to_string(), user_id],
    )?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_db() {
        create_db().unwrap();
    }

    #[test]
    fn test_create_connection() {
        create_connection().unwrap();
    }

    #[test]
    fn test_insert_user() {
        let result = insert_user("1234");
        assert!(result.is_ok());
    }

    #[test]
    fn test_insert_category() {
        insert_category("test_category_name").unwrap();
    }

    #[test]
    fn test_select_category() {
        let category_id = select_category("test_category_name").unwrap();
        assert_eq!(category_id, 1);
    }

    #[test]
    fn test_insert_homework() {
        insert_homework(
            "test_homework_name",
            "test_description",
            "2023-04-12",
            &1,
            "1234",
        )
        .unwrap();
    }

    #[test]
    fn test_select_all_categories() {
        let categories = select_all_categories().unwrap();
        let contains = categories.contains(&"test_category_name".to_string());
        assert_eq!(contains, true);
    }
}
