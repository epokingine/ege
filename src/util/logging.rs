use log::{set_logger, set_max_level, warn, Log};

struct EGELogger;

impl Log for EGELogger {
    fn enabled(&self, _metadata: &log::Metadata) -> bool {
        #[cfg(feature = "logging_disabled")]
        return false;

        return true;
    }

    fn log(&self, record: &log::Record) {
        if self.enabled(record.metadata()) {
            if record.file().is_none() || record.line().is_none() {
                println!("Missing file name and line number | {}", record.args());
            } else {
                println!(
                    "{} {}:{} {}",
                    record.level(),
                    record.file().unwrap(),
                    record.line().unwrap(),
                    record.args()
                );
            }
        }
    }

    fn flush(&self) {
        todo!("Implement")
    }
}

static EGELOGGER: EGELogger = EGELogger;

pub fn setup_logging() {
    if set_logger(&EGELOGGER).is_err() {
        warn!("Logger already setup");
    }

    set_max_level(log::LevelFilter::Info);
}
