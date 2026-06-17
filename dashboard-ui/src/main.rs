#[macro_use] extern crate rocket;

use crate::controller::html_get_dashboard::dashboard;
use crate::controller::rest_get_version::version;
use common::rocket::logging::log_format::LogFormat;
use rocket::fs::FileServer;
use tracing_subscriber::fmt;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

pub mod retrieve;
pub mod template;
pub mod cfg;
pub mod controller;



#[launch]
fn rocket() -> _ {
    tracing_subscriber::registry()
        .with(fmt::layer().with_ansi(true).event_format(LogFormat))
        .init();

    rocket::build()
        .mount("/", routes![dashboard])
        .mount("/static", FileServer::from("static"))
        .mount("/", routes![version])
}
