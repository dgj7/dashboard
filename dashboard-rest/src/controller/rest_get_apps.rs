use rocket::get;
use common::model::response::DashboardResponse;
use crate::data::query_apps::select_apps_by_owner;
use crate::data::query_envs::select_envs;
use crate::data::query_links::select_links_by_app_and_env;
use crate::session::query_user::determine_current_user;

// todo: this method should return a result, and all these expect() should be removed and replaced with ?
#[get("/maintainer/apps")]
pub fn maintainer_apps() -> String {
    let current_user = determine_current_user().expect("failed to query current user");
    let mut apps = select_apps_by_owner(&current_user).expect("failed to query apps");
    let mut envs = select_envs().expect("failed to query envs");
    for app in &mut apps {
        for env in &mut envs {
            let mut links = select_links_by_app_and_env(&app, &env).expect("failed to query links");
            if !links.is_empty() {
                let mut copy = env.clone();
                copy.links.append(&mut links);
                app.environments.push(copy);
            }
        }
    }
    serde_json::to_string(&DashboardResponse {apps}).unwrap()
}
