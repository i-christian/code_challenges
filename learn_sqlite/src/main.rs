use sqlite::{self, Connection};

fn main() {
    let connection = sqlite::open("sample.db").expect("Failed to initialise dabase");
    create_users(&connection);
    println!("Print a user before an update");
    get_user(&connection, 2);
    update_user(&connection, 2);
    println!("Print a user after an update");
    get_user(&connection, 2);

    println!("Print all users before a deletion:");
    get_users(&connection);

    delete_user(&connection, 1);
    println!("Print all users after a deletion");
    get_users(&connection);
}

// Create users
fn create_users(con: &Connection) {
    let query = "
        CREATE TABLE IF NOT EXISTS users (id INTEGER PRIMARY KEY ASC, name TEXT, age INTEGER);
        INSERT INTO users(name, age) VALUES('Innocent', 22);
        INSERT INTO users(name, age) VALUES('Christian', 26);
    ";

    con.execute(query).expect("failed to run querry");
}

// Retrieve all users in the database
fn get_users(con: &Connection) {
    let query = "SELECT * FROM users";

    for user in con
        .prepare(query)
        .unwrap()
        .into_iter()
        .map(|user| user.unwrap())
    {
        println!("name = {}", user.read::<&str, _>("name"));
        println!("age = {}", user.read::<i64, _>("age"));
    }
}

// Get a single user given an id
fn get_user(con: &Connection, id: i64) {
    let query = "SELECT * FROM users WHERE id = ?";

    for user in con
        .prepare(query)
        .unwrap()
        .into_iter()
        .bind((1, id))
        .unwrap()
        .map(|user| user.unwrap())
    {
        println!("name = {}", user.read::<&str, _>("name"));
        println!("age = {}", user.read::<i64, _>("age"));
    }
}

// Update a user given an id
fn update_user(con: &Connection, id: i64) {
    let query = format!(
        "UPDATE users SET name = '{}', age = {:?} WHERE id = {:?}",
        "Boruto", 16, id
    );
    con.execute(query).expect("Failed to update");
}

// Delete a user given an id
fn delete_user(con: &Connection, id: i64) {
    let query = format!(
        "
            DELETE FROM users WHERE id = {:?}
        ",
        id
    );
    con.execute(query).expect("Failed to delete user");
}
