use std::error::Error;
use common::model::app::Application;
use crate::data::DATABASE;

static QUERY_APPS: &str = "select a.id, a.name
from applications a
    inner join ownership w on w.app_id = a.id
    inner join owner o on o.id = w.owner_id
where o.name = ?";

pub fn select_apps_by_owner(owner: &str) -> Result<Vec<Application>,Box<dyn Error>> {
    let guard = DATABASE.lock()?;
    let mut statement = guard.prepare(QUERY_APPS)?;
    let apps = statement.query_map(&[&owner], |row| {
        Ok(Application {id: row.get(0)?, name: row.get(1)?, links: vec!()})
    })?.collect::<Result<Vec<Application>,_>>()?;
    Ok(apps)
}
