#[macro_use] extern crate rocket;

use rocket::fs::FileServer;
use template::dashboard_template::DashboardTemplate;
use crate::retrieve::retrieve_apps::retrieve_apps;

pub mod retrieve;
pub mod template;

#[get("/dashboard")]
async fn dashboard() -> DashboardTemplate {
    let response = retrieve_apps();

    match response.await {
        Ok(apps) => DashboardTemplate::new(apps),
        Err(_) => panic!("todo: replace with error template") // todo
    }
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![dashboard])
        .mount("/static", FileServer::from("static"))
}
