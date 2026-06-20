use std::time::Instant;
use rocket::State;
use common::health::app_state::ApplicationState;
use common::health::liveness::Liveness;
use common::rocket::uptime::UptimeTracker;

#[get("/ping")]
pub fn ping(start: &State<UptimeTracker>) -> String {
    let liveness = Liveness { application_state: ApplicationState::Up { started: start.started, elapsed_ms: Instant::now().duration_since(start.started).as_millis() } };
    serde_json::to_string(&liveness).unwrap()
}
