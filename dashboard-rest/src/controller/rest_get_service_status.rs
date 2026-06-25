use rocket::get;
use rocket::http::Status;
use common::status::svc_status::{ServiceStatus, ServiceStatusType};
use crate::retrieve::retrieve_service_status::retrieve_service_status;

pub const THIS_APP : & str = env!("CARGO_PKG_NAME");

///
/// get liveness for any service defined in the database.
///
#[get("/service?<name>&<env>")]
pub async fn service(name: &str, env: &str) -> Result<String,Status> {
    let status = if THIS_APP == name {
        ServiceStatus { status: ServiceStatusType::Up, message: "rest get service status: success".to_string() }
    } else {
        let status_result = retrieve_service_status(name, env).await;
        status_result.unwrap_or_else(|err| ServiceStatus { status: ServiceStatusType::Error, message: err.to_string() })
    };
    Ok(serde_json::to_string(&status).unwrap())
}
