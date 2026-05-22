#[macro_use] extern crate rocket;

use crate::hello_template::HelloTemplate;

pub mod hello_template;

#[get("/<name>")]
fn hello(name: &'_ str) -> HelloTemplate<'_> {
    HelloTemplate { name }
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![hello])
}
