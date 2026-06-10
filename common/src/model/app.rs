use serde::{Deserialize, Serialize};
use crate::model::environment::Environment;

#[derive(Serialize,Deserialize)]
pub struct Application {
    pub id: u32,
    pub name: String,
    pub environments: Vec<Environment>,
}
