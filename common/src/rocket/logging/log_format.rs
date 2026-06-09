use tracing::Subscriber;
use tracing_subscriber::fmt::{FmtContext, FormatEvent, FormatFields};
use tracing_subscriber::fmt::format::Writer;
use tracing_subscriber::registry::LookupSpan;
use std::fmt::Result;
use crate::rocket::logging::msg_visitor::MessageVisitor;

pub struct LogFormat;

impl <S,N> FormatEvent<S,N> for LogFormat
where
    S: Subscriber + for<'a> LookupSpan<'a>,
    N: for<'a> FormatFields<'a> + 'static,
{
    fn format_event(&self,
                    _ctx: &FmtContext<'_, S, N>,
                    mut writer: Writer<'_>,
                    event: &tracing_core::event::Event<'_>
    ) -> Result {
        let metadata = event.metadata();

        let mut message = String::new();
        let mut visitor = MessageVisitor { message: &mut message };
        event.record(&mut visitor);

        writeln!(writer, "{}: {}", metadata.level(), message)?;
        Ok(())
    }
}
