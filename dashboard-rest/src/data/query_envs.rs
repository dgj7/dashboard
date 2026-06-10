use std::error::Error;
use common::model::environment::Environment;
use crate::data::DATABASE;

static QUERY_ENVS: &str = "select e.id, e.name
from environments e";

pub fn select_envs() -> Result<Vec<Environment>,Box<dyn Error>> {
    let guard = DATABASE.lock()?;
    let mut statement = guard.prepare(QUERY_ENVS)?;
    let envs = statement.query_map([], |row| {
        Ok(Environment { id: row.get(0)?, name: row.get(1)?, links: vec!() })
    })?.collect::<Result<Vec<Environment>,_>>()?;
    Ok(envs)
}
