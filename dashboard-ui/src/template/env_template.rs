use common::model::app::Application;
use common::model::environment::Environment;

pub(crate) struct EnvironmentTemplate {
    pub(crate) name: String,
    pub(crate) dev_id: String,
}

impl EnvironmentTemplate {
    pub fn new(a: &Application, e: &Environment) -> Self {
        EnvironmentTemplate { name: format!("{}", e.name), dev_id: format!("{}_{}", e.name, a.name) }
    }

    pub fn all_from(a: &Application) -> Vec<EnvironmentTemplate> {
        let mut vec = vec!();
        for env in &a.environments {
            let et = EnvironmentTemplate::new(&a, &env);
            vec.push(et);
        }
        vec
    }
}
