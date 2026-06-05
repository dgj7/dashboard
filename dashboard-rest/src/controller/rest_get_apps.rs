use rocket::get;
use common::model::response::Response;
use crate::data::query_apps::select_apps_by_owner;
use crate::data::query_links::select_links_by_app;

#[get("/maintainer/apps?<owner>")]
pub fn maintainer_apps(owner: String) -> String {
    let mut apps = select_apps_by_owner(&*owner).expect("failed to query apps");
    for app in &mut apps {
        let mut links = select_links_by_app(&app).expect("failed to query links");
        app.links.append(&mut links);
    }
    serde_json::to_string(&Response {apps}).unwrap()
}
