use std::fmt::Debug;
use tracing_core::Field;

pub struct EventVisitor {
    pub message: String,
    pub file: String,
    pub line: String,
    pub target: String,
    pub module: String,
}

impl tracing::field::Visit for EventVisitor {
    fn record_str(&mut self, field: &Field, value: &str) {
        match field.name() {
            "message" => self.message.push_str(value),
            "log.line" => self.line.push_str(value),
            "log.file" => self.file.push_str(value),
            "log.target" => self.target.push_str(value),
            "log.module_path" => self.module.push_str(value),
            _ => {}
        }
    }

    fn record_debug(&mut self, field: &Field, value: &dyn Debug) {
        match field.name() {
            "message" => self.message.push_str(format!("{:?}", value).as_str()),
            "log.line" => self.line.push_str(format!("{:?}", value).as_str()),
            "log.file" => self.file.push_str(format!("{:?}", value).as_str()),
            "log.target" => self.target.push_str(format!("{:?}", value).as_str()),
            "log.module_path" => self.module.push_str(format!("{:?}", value).as_str()),
            _ => {}
        }
    }
}
