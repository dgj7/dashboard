use serde::{Deserialize, Serialize};

#[derive(Serialize,Deserialize)]
pub enum ServiceStatusType {
    Up,
    Error,
    Timeout,
}

#[derive(Serialize,Deserialize)]
pub struct ServiceStatus {
    pub status: ServiceStatusType,
}
