use common::model::app::Application;

pub(crate) struct AppTemplate {
    pub app_name: String,
}

impl AppTemplate {
    pub fn new(a: Application) -> Self {
        AppTemplate { app_name: a.name.clone() }
    }
}
