use std::time::Instant;
use serde::{Deserialize, Serialize};

#[derive(Serialize,Deserialize)]
pub enum ApplicationLiveness {
    Up {
        #[serde(with = "serde_millis")]
        started: Instant,
        elapsed_ms: u128 },
    Down(),
}
