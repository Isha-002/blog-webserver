use chrono::Local;
use tracing_subscriber::fmt::time::FormatTime;

pub struct CustomTimer;

impl FormatTime for CustomTimer {
    fn format_time(&self, w: &mut tracing_subscriber::fmt::format::Writer<'_>) -> std::result::Result<(), std::fmt::Error> {
        let now = Local::now();
        write!(
            w,
            "{}",
            now.format("%d/%m/%Y %H:%M:%S")
        )
    }
}