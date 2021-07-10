use tracing::subscriber::set_global_default;
use tracing_appender::{non_blocking, rolling};
use tracing_log::LogTracer;
use tracing_subscriber::{fmt, layer::SubscriberExt, EnvFilter, Registry};

use crate::settings::LogSettings;

pub struct LoggingGuard {
    _non_blocking: non_blocking::WorkerGuard,
}

pub fn initialize(settings: &LogSettings) -> LoggingGuard {
    let file_appender = rolling::daily(&settings.directory, "image-resize-api.log");
    let (non_blocking, non_blocking_guard) = non_blocking(file_appender);

    let subscriber = Registry::default()
        .with(EnvFilter::new(&settings.directive))
        .with(fmt::layer().with_ansi(false).with_writer(non_blocking))
        .with(fmt::layer().pretty());

    LogTracer::init().expect("Failed to set logger");

    set_global_default(subscriber).expect("Failed to set subscriber");

    LoggingGuard {
        _non_blocking: non_blocking_guard,
    }
}
