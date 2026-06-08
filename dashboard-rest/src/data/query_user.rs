use common::model::auth::user::CurrentUser;

pub fn select_current_user() -> Option<CurrentUser> {
    // todo: this needs to actually pull the currently logged in user
    Some(CurrentUser { id: 1, name: "fronk".to_string() })
}
