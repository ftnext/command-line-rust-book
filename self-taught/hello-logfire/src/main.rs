// https://github.com/pydantic/logfire-rust/blob/v0.8.2/src/logfire.rs#L129-L144
use logfire::config::ConsoleOptions;
use tracing::{Level, event, instrument};
use tracing_subscriber::layer::SubscriberExt;

#[instrument]
fn my_function(my_arg: usize) {
    event!(Level::INFO, "doing some interesting work");
    let x = 42i64;
    let y = "spam";
    event!(Level::INFO, "Hello, world! x = {}, y = {}", x, y);
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let logfire = logfire::configure()
        .local()
        .with_console(Some(ConsoleOptions::default()))
        .finish()
        .expect("Failed to configure logfire");

    // local() の上で macro を記録するための設定
    let _guard = logfire::set_local_logfire(logfire.clone());
    logfire::info!("Hello world!");

    let subscriber = tracing_subscriber::registry().with(logfire.tracing_layer());
    tracing::subscriber::set_global_default(subscriber).expect("Failed to set global subscriber");
    my_function(1);

    logfire.shutdown().expect("Failed to shutdown logfire");
    Ok(())
}
