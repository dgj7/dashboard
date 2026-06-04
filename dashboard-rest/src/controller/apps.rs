use rocket::get;
use common::model::response::Response;

#[get("/maintainer/apps")]
pub fn maintainer_apps() -> String {
    let response = Response::new();
    serde_json::to_string(&response).unwrap()
}
