use common::model::response::DashboardResponse;

pub(crate) async fn retrieve_apps() -> Result<DashboardResponse,rocket::http::Status> {
    // todo: this url needs to be externalized; maybe config or toml crates
    let response = reqwest::get("http://127.0.0.1:8080/maintainer/apps".to_string())
        .await
        .map_err(|_| rocket::http::Status::InternalServerError)?;

    if response.status().is_success() {
        response.json::<DashboardResponse>().await.map_err(|_| rocket::http::Status::InternalServerError)
    } else {
        Err(rocket::http::Status::new(response.status().as_u16()))
    }
}
