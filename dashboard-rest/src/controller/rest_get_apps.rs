use rocket::get;
use common::model::response::Response;
use crate::data::query_apps::select_apps_by_owner;
use crate::data::query_links::select_links_by_app;
use crate::data::query_user::select_current_user;

#[get("/maintainer/apps")]
pub fn maintainer_apps() -> String {
    let current_user = select_current_user().expect("failed to query current user");
    println!("current_user.id={}", current_user.id);
    let mut apps = select_apps_by_owner(&current_user).expect("failed to query apps");
    for app in &mut apps {
        let mut links = select_links_by_app(&app).expect("failed to query links");
        app.links.append(&mut links);
    }
    serde_json::to_string(&Response {apps}).unwrap()
}
