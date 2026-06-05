use rocket::get;
use crate::data::select_apps_by_owner;

#[get("/maintainer/apps?<owner>")]
pub fn maintainer_apps(owner: String) -> String {
    let response = select_apps_by_owner(&*owner).expect("failed");
    serde_json::to_string(&response).unwrap()
}
