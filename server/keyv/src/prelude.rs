use std::str::FromStr;
use tracing_subscriber::EnvFilter;
use tracing_subscriber::fmt::format;
use tracing_subscriber::util::SubscriberInitExt;

pub type Res = eyre::Result<()>;

#[inline]
pub fn init_logger(level: &str) {
    tracing_subscriber::fmt::fmt()
        .pretty()
        .with_level(true)
        .with_env_filter(EnvFilter::from_str(level).unwrap())
        .event_format(
            format()
                .pretty()
                .compact()
                .with_level(true)
                .with_target(true)
                .with_thread_ids(true)
                .with_source_location(cfg!(debug_assertions))
                .with_thread_names(true)
        )
        .finish()
        .init();
}
