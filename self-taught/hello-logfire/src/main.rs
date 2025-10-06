use logfire::config::ConsoleOptions;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let logfire = logfire::configure()
        .with_console(Some(ConsoleOptions::default()))
        .finish()?;

    let _guard = logfire.shutdown_guard();

    logfire::info!("Hello, world!");

    Ok(())
}
