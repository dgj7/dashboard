use serde::{Deserialize, Serialize};
use crate::model::app::Application;

#[derive(Serialize,Deserialize)]
pub struct Response {
    pub apps: Vec<Application>,
}
