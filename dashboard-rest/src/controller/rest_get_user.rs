use rocket::get;
use common::model::auth::user::CurrentUser;
use crate::data::query_user::select_current_user;

#[get("/current_user")]
pub fn current_user() -> String {
    let user = select_current_user().unwrap_or_else(|| { CurrentUser::guest() });
    serde_json::to_string(&user).unwrap()
}
