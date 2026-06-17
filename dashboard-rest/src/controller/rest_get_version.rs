use rocket::get;
use crate::retrieve::retrieve_version::retrieve_dashrest_version;

#[get("/version")]
pub fn version() -> String {
    let version = retrieve_dashrest_version();
    serde_json::to_string(&version).unwrap()
}
