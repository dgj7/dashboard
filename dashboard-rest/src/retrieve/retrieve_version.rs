use common::info::version::Version;

pub fn retrieve_dashrest_version() -> Version {
    Version {
        app_name: env!("CARGO_PKG_NAME").to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
        commit: env!("GIT_COMMIT").to_string()
    }
}
