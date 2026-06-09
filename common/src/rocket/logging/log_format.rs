use tracing::Subscriber;
use tracing_subscriber::fmt::{FmtContext, FormatEvent, FormatFields};
use tracing_subscriber::fmt::format::Writer;
use tracing_subscriber::registry::LookupSpan;
use std::fmt::Result;
use std::path::Path;
use std::time::SystemTime;
use chrono::DateTime;
use owo_colors::{Style, Styled};
use tracing_core::Level;
use crate::rocket::logging::event_visitor::EventVisitor;

pub struct LogFormat;

static PRE: Style = Style::new().magenta();
static TAR: Style = Style::new().green();
static BEFORE: usize = 40;
static AFTER: usize = BEFORE + 5;

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
        let visitor = visit(event);
        let filename = Path::new(&visitor.file)
            .file_name()
            .and_then(|s| s.to_str())
            .unwrap_or("unknown");
        let target = TAR.style(format!("{:BEFORE$.BEFORE$}", format!("{};{}#{}", &visitor.module, filename, visitor.line)));
        writeln!(writer, "{:19.19} {:5.5} {:AFTER$.AFTER$} {} {}", time(), level(event), target, PRE.style("|"), visitor.message)?;
        Ok(())
    }
}

fn time() -> Styled<String> {
    let ts_boring = DateTime::<chrono::Utc>::from(SystemTime::now())
        .format("%Y-%m-%d %H:%M:%S")
        .to_string();
    Style::new()
        .blue()
        .bold()
        .style(ts_boring)
}

fn level(event: &tracing_core::event::Event<'_>) -> Styled<Level> {
    Style::new()
        .yellow()
        .style(*event.metadata().level())
}

fn visit(event: &tracing_core::event::Event<'_>) -> EventVisitor {
    let mut visitor = EventVisitor { message: String::new(), file: String::new(), line: String::new(), target: String::new(), module: String::new() };
    event.record(&mut visitor);
    visitor
}
