use rocket::{get};
use common::status::svc_status::{ServiceStatus, ServiceStatusType};
use crate::retrieve::retrieve_service_status::retrieve_service_status;

pub const THIS_APP : & str = env!("CARGO_PKG_NAME");

///
/// get liveness for any service defined in the database.
///
#[get("/service?<name>&<env>")]
pub async fn service(name: &str, env: &str) -> String {// todo: this service should return Result<>
    let status = if THIS_APP == name {
        ServiceStatus { status: ServiceStatusType::Up }
    } else {
        retrieve_service_status(name, env).await
    };
    serde_json::to_string(&status).unwrap()
}
