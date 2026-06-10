use serde::{Deserialize, Serialize};
use crate::model::link::Link;

#[derive(Serialize,Deserialize,Clone)]
pub struct Environment {
    pub id: u32,
    pub name: String,
    pub links: Vec<Link>,
}