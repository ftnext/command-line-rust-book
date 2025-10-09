// use logfire::config::ConsoleOptions;
use tracing::{Level, event, instrument};

#[instrument]
fn my_function(my_arg: usize) {
    event!(Level::INFO, "doing some interesting work");
    let x = 42i64;
    let y = "spam";
    event!(Level::INFO, "Hello, world! x = {}, y = {}", x, y);
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let logfire = logfire::configure()
        // .with_console(Some(ConsoleOptions::default()))
        .finish()?;
    let _guard = logfire.shutdown_guard();

    my_function(1);
    Ok(())
}
