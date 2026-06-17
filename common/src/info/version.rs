use serde::{Deserialize, Serialize};

#[derive(Serialize,Deserialize)]
pub struct Version {
    pub app_name: String,
    pub version: String,
    pub commit: String,
}
