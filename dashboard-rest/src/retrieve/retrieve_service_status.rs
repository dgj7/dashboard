use crate::data::query_service_status::query_service_liveness_url;
use common::status::svc_status::{ServiceStatus, ServiceStatusType};
use reqwest::Client;
use std::time::Duration;

pub async fn retrieve_service_status(app_name: &str, env_name: &str) -> ServiceStatus {
    let url = query_service_liveness_url(app_name, env_name).unwrap();

    // todo: this client is reusable; repackage this similarly to database access handle
    let client = Client::builder()
        .timeout(Duration::from_secs(2))// todo: this timeout should be externalized
        .connect_timeout(Duration::from_secs(1)) // todo: this timeout should be externalized
        .build()
        .unwrap();// todo: don't panic here; this function should return Result<>

    // todo: this url needs to be externalized; maybe config or toml crates
    let response = client.get(url)
        .send()
        .await;

    match response {
        Ok(_) => { ServiceStatus { status: ServiceStatusType::Up } }
        Err(err) => {
            if err.is_timeout() {
                ServiceStatus { status: ServiceStatusType::Timeout }
            } else {
                ServiceStatus { status: ServiceStatusType::Error }
            }
        }
    }
}
