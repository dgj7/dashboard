use rocket::{launch, routes};
use controller::rest_get_liveness::ping;
use crate::controller::rest_get_apps::maintainer_apps;
use crate::controller::rest_get_user::current_user;

pub mod controller;
pub mod data;
pub mod session;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![ping])
        .mount("/", routes![maintainer_apps])
        .mount("/", routes![current_user])
}
