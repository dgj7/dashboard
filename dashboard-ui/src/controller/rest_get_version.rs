use crate::retrieve::retrieve_version::retrieve_dashui_version;

#[get("/version")]
pub fn version() -> String {
    let version = retrieve_dashui_version();
    serde_json::to_string(&version).unwrap()
}
