use serde::{Deserialize, Serialize};
use crate::health::app_state::ApplicationState;

#[derive(Serialize,Deserialize)]
pub struct Liveness {
    pub application_state: ApplicationState,
}
