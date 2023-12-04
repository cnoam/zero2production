//! main.rs

use zero2prod::configuration::get_configuration;
use zero2prod::telemetry::{get_subscriber, init_subscriber};
use zero2prod::startup::{Application};

fn more(x: u32) -> u32{
    x + 100
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    print!("{}\n",more(4294967280));
    let subscriber = get_subscriber("zero2prod".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);

    Application::build(
        get_configuration().expect("Failed to read configuration.")).await?.run_until_stopped().await?;
    Ok(())
}
