use anyhow::Context;
use logfire::{
    __macros_impl::Level,
    ShutdownGuard,
    config::{AdvancedOptions, ConsoleOptions}
};

pub fn configure() -> anyhow::Result<ShutdownGuard> {
    let otel_endpoint = std::env::var("OTEL_EXPORTER_OTLP_ENDPOINT")
        .context("`OTEL_EXPORTER_OTLP_ENDPOINT` must be set")?;
    let options = AdvancedOptions::default().with_base_url(otel_endpoint);
    let logfire = logfire::configure()
        .send_to_logfire(false)
        .with_advanced_options(options)
        .with_console(Some(
            ConsoleOptions::default().with_min_log_level(Level::TRACE)
        ))
        .finish()?;
    let guard = logfire.shutdown_guard();
    logfire::info!("observability has been configured");
    Ok(guard)
}
