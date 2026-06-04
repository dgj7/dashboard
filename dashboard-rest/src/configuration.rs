use rusqlite::Connection;
use std::sync::{Mutex, OnceLock};

pub static DATABASE: OnceLock<Mutex<Connection>> = OnceLock::new();

pub fn database() -> &'static Mutex<Connection> {
    DATABASE.get_or_init(|| {
        let conn = Connection::open("apps.db").unwrap();
        Mutex::new(conn)
    })
}
