use crate::data::DATABASE;
use std::error::Error;

static QUERY_SERVICE: &str = "select l.url
from applications a
inner join main.link l on a.id = l.app_id
inner join main.link_type lt on l.link_type = lt.id
inner join main.environments e on e.id = l.env_id
where
    lt.name = ?
    and a.name = ?
    and e.name = ?";

pub fn query_service_liveness_url(app_name: &str, env_name: &str) -> Result<String,Box<dyn Error>> {
    let guard = DATABASE.lock()?;
    let mut statement = guard.prepare(QUERY_SERVICE)?;
    let links_iter = statement.query_map(&["liveness", app_name, env_name], |row| {
        let url: String = row.get::<usize, String>(0)?;
        Ok(url)
    })?;
    let links: Vec<String> = links_iter.collect::<Result<_, _>>()?;
    if links.len() > 1 || links.is_empty() {
        panic!("bad query: expected 1 link, got {}", links.len());
    }
    Ok(links.first().unwrap().to_string())
}
