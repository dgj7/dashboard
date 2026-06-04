use serde::{Deserialize, Serialize};

#[derive(Serialize,Deserialize)]
pub struct Link {
    pub id: u32,
    pub name: String,
    pub url: String,
}
