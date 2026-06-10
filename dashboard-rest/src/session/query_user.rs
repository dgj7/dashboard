use common::auth::user::CurrentUser;

pub fn determine_current_user() -> Option<CurrentUser> {
    // todo: this needs to actually pull the currently logged in user
    Some(CurrentUser { id: 300, name: "fronk".to_string() })
}
