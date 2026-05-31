#[macro_use] extern crate rocket;

use rocket::fs::FileServer;
use crate::dashboard_template::DashboardTemplate;

mod dashboard_template;

#[get("/dashboard")]
fn dashboard() -> DashboardTemplate {
    DashboardTemplate {}
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![dashboard])
        .mount("/static", FileServer::from("static"))
}
