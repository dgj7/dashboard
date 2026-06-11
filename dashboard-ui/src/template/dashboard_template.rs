use askama::Template;
use rocket::http::Status;
use rocket::response::{content, Responder};
use rocket::{response, Request};
use common::model::app::Application;
use common::model::response::DashboardResponse;

#[derive(Template)]
#[template(path = "dashboard.html")]
pub(crate) struct DashboardTemplate {
    pub apps: Vec<Application>,
}

impl<'r> Responder<'r, 'static> for DashboardTemplate {
    fn respond_to(self, req: &'r Request<'_>) -> response::Result<'static> {
        let html = self.render().map_err(|_| Status::InternalServerError)?;
        content::RawHtml(html).respond_to(req)
    }
}

impl DashboardTemplate {
    pub fn new(dashboard_response: DashboardResponse) -> Self {
        DashboardTemplate { apps: dashboard_response.apps }
    }
}
