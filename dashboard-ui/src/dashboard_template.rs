use askama::Template;
use rocket::http::Status;
use rocket::response::{content, Responder};
use rocket::{response, Request};

#[derive(Template)]
#[template(path = "dashboard.html")]
pub(crate) struct DashboardTemplate {
}

impl<'r> Responder<'r, 'static> for DashboardTemplate {
    fn respond_to(self, req: &'r Request<'_>) -> response::Result<'static> {
        let html = self.render().map_err(|_| Status::InternalServerError)?;
        content::RawHtml(html).respond_to(req)
    }
}
