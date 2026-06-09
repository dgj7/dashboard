#[macro_use] extern crate rocket;

use rocket::fs::FileServer;
use tracing_subscriber::fmt;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use template::dashboard_template::DashboardTemplate;
use common::rocket::logging::log_format::LogFormat;
use crate::retrieve::retrieve_apps::retrieve_apps;

pub mod retrieve;
pub mod template;
pub mod cfg;

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
    tracing_subscriber::registry()
        .with(fmt::layer().with_ansi(true).event_format(LogFormat))
        .init();

    rocket::build()
        .mount("/", routes![dashboard])
        .mount("/static", FileServer::from("static"))
}
