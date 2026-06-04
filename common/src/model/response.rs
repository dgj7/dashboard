use serde::{Deserialize, Serialize};
use crate::model::app::Application;

#[derive(Serialize,Deserialize)]
pub struct Response {
    pub apps: Vec<Application>,
}

impl Response {
    pub fn new() -> Self {
        Self { apps: vec![] }
    }
}
