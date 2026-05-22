#[macro_use] extern crate rocket;

use crate::hello_template::HelloTemplate;

pub mod hello_template;

#[get("/<name>")]
fn hello(name: &str) -> HelloTemplate {
    HelloTemplate { name: name.parse().unwrap() }
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![hello])
}
