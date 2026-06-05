use std::error::Error;
use rusqlite::Connection;
use std::sync::{LazyLock, Mutex};
use common::model::app::Application;
use common::model::response::Response;

pub static DATABASE: LazyLock<Mutex<Connection>> = LazyLock::new(|| {
    Mutex::new(Connection::open("apps.db").unwrap())
});

static QUERY_APPS: &str = "select a.id, a.name
from applications a
    inner join ownership w on w.app_id = a.id
    inner join owner o on o.id = w.owner_id
where o.name = ?";

pub fn select_apps_by_owner(owner: &str) -> Result<Response,Box<dyn Error>> {
    let guard = DATABASE.lock()?;
    let mut statement = guard.prepare(QUERY_APPS)?;
    let apps = statement.query_map(&[&owner], |row| {
        Ok(Application {id: row.get(0)?, name: row.get(1)?, links: vec!()})
    })?.collect::<Result<Vec<Application>,_>>()?;
    Ok(Response { apps })
}
