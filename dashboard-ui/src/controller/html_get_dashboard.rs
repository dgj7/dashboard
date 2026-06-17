use crate::retrieve::retrieve_apps::retrieve_apps;
use crate::template::dashboard_template::DashboardTemplate;

#[get("/dashboard")]
pub async fn dashboard() -> DashboardTemplate {
    let response = retrieve_apps();

    match response.await {
        Ok(apps) => DashboardTemplate::new(apps),
        Err(_) => panic!("todo: replace with error template") // todo
    }
}
