use logfire::ShutdownGuard;

pub fn configure() -> anyhow::Result<ShutdownGuard> {
    let logfire = logfire::configure().finish()?;
    let guard = logfire.shutdown_guard();
    Ok(guard)
}