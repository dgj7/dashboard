pub mod query_apps;
pub mod query_links;
pub mod query_envs;
pub mod query_service_status;

use rusqlite::Connection;
use std::sync::{LazyLock, Mutex};

pub static DATABASE: LazyLock<Mutex<Connection>> = LazyLock::new(|| {
    Mutex::new(Connection::open("../apps.db").unwrap())
});
