use tracing::{Level, event, instrument};

#[instrument]
fn my_function(my_arg: usize) {
    event!(Level::INFO, "doing some interesting work");
    let x = 42i64;
    let y = "spam";
    event!(Level::INFO, "Hello, world! x = {}, y = {}", x, y);
}

fn main() {
    tracing_subscriber::fmt().init();

    my_function(1);
}
