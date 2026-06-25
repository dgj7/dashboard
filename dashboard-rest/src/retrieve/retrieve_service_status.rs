use std::error::Error;
use crate::data::query_service_status::query_service_liveness_url;
use common::status::svc_status::{ServiceStatus, ServiceStatusType};
use reqwest::Client;
use std::time::Duration;

pub async fn retrieve_service_status(app_name: &str, env_name: &str) -> Result<ServiceStatus,Box<dyn Error>> {
    let url = query_service_liveness_url(app_name, env_name)?;

    // todo: this client is reusable; repackage this similarly to database access handle
    let client = Client::builder()
        .timeout(Duration::from_secs(2))// todo: this timeout should be externalized
        .connect_timeout(Duration::from_secs(1)) // todo: this timeout should be externalized
        .build()?;

    let response = client.get(url)
        .send()
        .await;

    match response {
        Ok(_) => { Ok(ServiceStatus { status: ServiceStatusType::Up, message: "retrieve_service_status successful".to_string() }) },
        Err(err) => {
            if err.is_timeout() {
                Ok(ServiceStatus { status: ServiceStatusType::Timeout, message: err.to_string() })
            } else {
                Ok(ServiceStatus { status: ServiceStatusType::Error, message: err.to_string() })
            }
        }
    }
}
