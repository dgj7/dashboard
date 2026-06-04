use serde::{Deserialize, Serialize};
use crate::model::link::Link;

#[derive(Serialize,Deserialize)]
pub struct Application {
    pub id: u32,
    pub name: String,
    pub links: Vec<Link>,
}
