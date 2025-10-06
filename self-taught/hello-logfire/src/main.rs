use logfire::config::ConsoleOptions;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let logfire = logfire::configure()
        .with_console(Some(ConsoleOptions::default()))
        .finish()?;

    let _guard = logfire.shutdown_guard();

    logfire::span!("doing some interesting work").in_scope(|| {
        let x = 42i64;
        let y = "spam";
        logfire::info!("Hello, world! x = {x}, y = {y}", x, y);
        panic!("Oh no!");
    });

    Ok(())
}
