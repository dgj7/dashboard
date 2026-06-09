use serde::{Deserialize, Serialize};
use crate::model::app::Application;

#[derive(Serialize,Deserialize)]
pub struct DashboardResponse {
    pub apps: Vec<Application>,
}
