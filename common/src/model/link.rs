use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct Link {
    pub id: u32,
    pub link_type: String,
    pub url: String,
}
