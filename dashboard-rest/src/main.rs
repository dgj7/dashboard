use rocket::{launch, routes};
use controller::liveness::ping;
use crate::controller::apps::maintainer_apps;

mod controller;
mod data;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![ping])
        .mount("/", routes![maintainer_apps])
}
