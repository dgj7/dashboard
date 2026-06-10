use std::error::Error;
use common::model::app::Application;
use common::model::environment::Environment;
use common::model::link::Link;
use crate::data::DATABASE;

static QUERY_LINKS: &str = "select l.id, t.name, l.url
from link l
    inner join applications a on a.id = l.app_id
    inner join link_type t on t.id = l.link_type
where a.id = ? and l.env_id = ?";

pub fn select_links_by_app_and_env(app: &Application, env: &Environment) -> Result<Vec<Link>,Box<dyn Error>> {
    let guard = DATABASE.lock()?;
    let mut statement = guard.prepare(QUERY_LINKS)?;
    let links = statement.query_map(&[&app.id, &env.id], |row| {
        Ok(Link {id: row.get(0)?, link_type: row.get(1)?, url: row.get(2)?})
    })?.collect::<Result<Vec<Link>,_>>()?;
    Ok(links)
}
