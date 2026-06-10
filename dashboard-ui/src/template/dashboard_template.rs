use askama::Template;
use rocket::http::Status;
use rocket::response::{content, Responder};
use rocket::{response, Request};
use common::model::response::DashboardResponse;
use crate::template::app_template::AppTemplate;

#[derive(Template)]
#[template(path = "dashboard.html")]
pub(crate) struct DashboardTemplate {
    pub apps: Vec<AppTemplate>,
}

impl<'r> Responder<'r, 'static> for DashboardTemplate {
    fn respond_to(self, req: &'r Request<'_>) -> response::Result<'static> {
        let html = self.render().map_err(|_| Status::InternalServerError)?;
        content::RawHtml(html).respond_to(req)
    }
}

impl DashboardTemplate {
    pub fn new(dbr : DashboardResponse) -> Self {
        let mut aa = vec!();
        for dba in dbr.apps {
            aa.push(AppTemplate::new(&dba));
        }
        DashboardTemplate { apps: aa }
    }
}
