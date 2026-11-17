use logfire::{ShutdownGuard, config::ConsoleOptions};

/// Configure ['logfire'] console logging + tracing.
/// Currently, this does not support sending traces/logs to
/// otel-collector.
pub fn configure() -> anyhow::Result<ShutdownGuard> {
    let logfire = logfire::configure()
        .send_to_logfire(false)
        .with_console(Some(ConsoleOptions::default()))
        .finish()?;
    let guard = logfire.shutdown_guard();
    logfire::info!("observability has been configured");
    Ok(guard)
}
