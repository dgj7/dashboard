use rocket::{launch, routes};
use controller::liveness::ping;

mod controller;
mod configuration;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![ping])
}
