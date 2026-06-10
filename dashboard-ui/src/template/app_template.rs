use common::model::app::Application;
use crate::template::env_template::EnvironmentTemplate;

pub(crate) struct AppTemplate {
    pub app_name: String,
    pub environments: Vec<EnvironmentTemplate>
}

impl AppTemplate {
    pub fn new(a: &Application) -> Self {
        AppTemplate { app_name: a.name.clone(), environments: EnvironmentTemplate::all_from(&a) }
    }
}
