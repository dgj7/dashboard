use serde::{Deserialize, Serialize};

///
/// PUBLIC user data.
/// 
/// does NOT include password, ever.
/// 
#[derive(Serialize,Deserialize)]
pub struct CurrentUser {
    pub id: i32,
    pub name: String,
}

impl CurrentUser {
    pub fn guest() -> CurrentUser {
        CurrentUser { id: 0, name: "guest".to_string() }
    }
}
