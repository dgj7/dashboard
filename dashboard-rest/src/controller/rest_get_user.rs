use rocket::get;
use common::model::auth::user::CurrentUser;
use crate::session::query_user::determine_current_user;

#[get("/current_user")]
pub fn current_user() -> String {
    let user = determine_current_user().unwrap_or_else(|| { CurrentUser::guest() });
    serde_json::to_string(&user).unwrap()
}
