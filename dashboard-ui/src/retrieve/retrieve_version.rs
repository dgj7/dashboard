use common::info::version::Version;

pub fn retrieve_dashui_version() -> Version {
    Version { 
        app_name: env!("CARGO_PKG_NAME").to_string(), 
        version: env!("CARGO_PKG_VERSION").to_string(), 
        commit: env!("GIT_COMMIT").to_string() 
    }
}
