const DEFAULT_LOG_SIZE: u64 = 7 * 1024 * 1024;
pub const DEFAULT_LOG_LEVEL: &str = "info";

pub static LOG_GUARD: once_cell::sync::Lazy<
    once_cell::sync::OnceCell<tracing_appender::non_blocking::WorkerGuard>,
    // once_cell::sync::OnceCell<()>,
> = once_cell::sync::Lazy::new(once_cell::sync::OnceCell::new);

pub fn init_log(_path: &str) -> Result<(), crate::Error> {
    LOG_GUARD.get_or_init(|| _init_log(DEFAULT_LOG_LEVEL, _path));

    Ok(())
}

fn _init_log(level: &str, path: &str) -> WorkerGuard {
    use tracing_subscriber::{
        fmt, prelude::__tracing_subscriber_SubscriberExt, util::SubscriberInitExt, EnvFilter,
        Registry,
    };
    let file_appender = tracing_appender::rolling::never(path, format!("{level}.log"));
    let (non_blocking, guard) = tracing_appender::non_blocking(file_appender);

    let env_filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new(level));
    let formatting_layer = fmt::layer()
        .pretty()
        .with_writer(std::io::stderr)
        // .with_writer(file_appender)
        .with_writer(non_blocking)
        .with_ansi(false);
    Registry::default()
        .with(env_filter)
        // ErrorLayer 可以让 color-eyre 获取到 span 的信息
        .with(tracing_error::ErrorLayer::default())
        .with(FileCheckLayer {
            log_file_path: format!("{path}/{level}.log"),
            size_limit: DEFAULT_LOG_SIZE,
        })
        .with(fmt::layer())
        .with(formatting_layer)
        .init();
    tracing::error!("[init log] Init log success");
    guard
}

struct FileCheckLayer {
    log_file_path: String,
    size_limit: u64,
}
use tracing::{Event, Subscriber};
use tracing_appender::non_blocking::WorkerGuard;
use tracing_subscriber::{layer::Context, Layer};

impl<S> Layer<S> for FileCheckLayer
where
    S: Subscriber + for<'a> tracing_subscriber::registry::LookupSpan<'a>,
{
    fn on_event(&self, _event: &Event<'_>, _ctx: Context<'_, S>) {
        // Check the file size each time the log is written
        if let Ok(metadata) = std::fs::metadata(&self.log_file_path) {
            tracing::debug!("Check File Size: {}", metadata.len());
            if metadata.len() > self.size_limit {
                let _ = std::fs::write(&self.log_file_path, b""); // Clear file
            }
        }
    }
}
