use std::fmt::Debug;
use tracing_core::Field;

pub(crate) struct MessageVisitor<'a> {
    pub(crate) message: &'a mut String,
}

impl<'a> tracing::field::Visit for MessageVisitor<'a> {
    fn record_str(&mut self, field: &Field, value: &str) {
        if field.name() == "message" {
            self.message.push_str(&value);
        }
    }

    fn record_debug(&mut self, field: &Field, value: &dyn Debug) {
        if field.name() == "message" {
            self.message.push_str(format!("{:?}", value).as_str());
        }
    }
}
