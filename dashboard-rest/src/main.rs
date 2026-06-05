use rocket::{launch, routes};
use controller::rest_get_liveness::ping;
use crate::controller::rest_get_apps::maintainer_apps;

mod controller;
mod data;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![ping])
        .mount("/", routes![maintainer_apps])
}
