use sqlite::{self, Connection, Row};

fn main() {
    let connection = sqlite::open("sample.db").expect("Failed to initialise dabase");
    create_users(&connection);
    get_user(&connection, 1);
    println!("Print all user before an update");
    get_user(&connection, 2);
    update_user(&connection, 2);
    println!("Print all user after an update");
    get_user(&connection, 2);

    println!("Print all users:");
    // get all users in the database
    let users = get_users(&connection);
    for user in users {
        println!("name = {}", user.read::<&str, _>("name"));
        println!("age = {}", user.read::<i64, _>("age"));
    }
}

fn create_users(con: &Connection) {
    let query = "
        CREATE TABLE IF NOT EXISTS users (id INTEGER PRIMARY KEY ASC, name TEXT, age INTEGER);
        INSERT INTO users(name, age) VALUES('Innocent', 22);
        INSERT INTO users(name, age) VALUES('Christian', 26);
    ";

    con.execute(query).expect("failed to run querry");
}

fn get_users(con: &Connection) -> Vec<Row> {
    let query = "SELECT * FROM users";
    let mut users = Vec::new();

    for user in con
        .prepare(query)
        .unwrap()
        .into_iter()
        .map(|user| user.unwrap())
    {
        users.push(user)
    }

    users
}

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

fn update_user(con: &Connection, id: i64) {
    let query = format!("UPDATE users SET name = Boruto WHERE id = {:?}", id);
    con.execute(query).expect("Failed to update");
}
