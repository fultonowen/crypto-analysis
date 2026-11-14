use logfire::{ShutdownGuard, config::AdvancedOptions};

pub fn configure() -> anyhow::Result<ShutdownGuard> {
    let logfire = logfire::configure()
        .send_to_logfire(false)
        .with_advanced_options(AdvancedOptions::default())
        .finish()?;
    let guard = logfire.shutdown_guard();
    logfire::info!("Tracing has been configured");
    Ok(guard)
}
