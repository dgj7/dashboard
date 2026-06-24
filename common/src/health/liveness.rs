use serde::{Deserialize, Serialize};
use crate::health::app_liveness::ApplicationLiveness;

#[derive(Serialize,Deserialize)]
pub struct Liveness {
    pub application_state: ApplicationLiveness,
}
