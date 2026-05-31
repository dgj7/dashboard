use crate::model::link::Link;

pub struct Application {
    pub id: u32,
    pub name: String,
    pub links: Vec<Link>,
}
