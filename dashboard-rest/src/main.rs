use rocket::{launch, routes};
use crate::liveness::ping;

mod liveness;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![ping])
}
