#[macro_use] extern crate rocket;

use rocket::fs::FileServer;
use crate::hello_template::HelloTemplate;

pub mod hello_template;

#[get("/<name>")]
fn hello(name: &str) -> HelloTemplate {
    HelloTemplate { name: name.parse().unwrap() }
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![hello])
        .mount("/static", FileServer::from("static"))
}
