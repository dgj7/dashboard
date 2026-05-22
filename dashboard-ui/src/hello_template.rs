use askama::Template;
use rocket::http::Status;
use rocket::{response, Request};
use rocket::response::{content, Responder};

#[derive(Template)]
#[template(path = "hello.html")]
pub(crate) struct HelloTemplate<'a> {
    pub(crate) name: &'a str,
}

impl<'r> Responder<'r, 'static> for HelloTemplate<'_> {
    fn respond_to(self, req: &'r Request<'_>) -> response::Result<'static> {
        let html = self.render().map_err(|_| Status::InternalServerError)?;
        content::RawHtml(html).respond_to(req)
    }
}
