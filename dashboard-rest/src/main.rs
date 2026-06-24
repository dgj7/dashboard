use std::time::Instant;
use rocket::{launch, routes};
use tracing_subscriber::fmt;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use common::rocket::logging::log_format::LogFormat;
use common::rocket::uptime::UptimeTracker;
use controller::rest_get_liveness::ping;
use crate::controller::rest_get_apps::maintainer_apps;
use crate::controller::rest_get_service_status::service;
use crate::controller::rest_get_user::current_user;
use crate::controller::rest_get_version::version;

pub mod controller;
pub mod data;
pub mod session;
pub mod retrieve;

#[launch]
fn rocket() -> _ {
    tracing_subscriber::registry()
        .with(fmt::layer().with_ansi(true).event_format(LogFormat))
        .init();

    rocket::build()
        .manage(UptimeTracker { started: Instant::now() })
        .mount("/", routes![ping])
        .mount("/", routes![maintainer_apps])
        .mount("/", routes![current_user])
        .mount("/", routes![version])
        .mount("/", routes![service])
}
