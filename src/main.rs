
#[tokio::main]

async fn main() ->  Result<(), Box<dyn std::error::Error>> {
    //initialize debugging
    tracing_subscriber::fmt().with_env_filter(tracing_subscriber::EnvFilter::from_default_env()).init();

    tracing::info!("Loyalty Engine Starting!");

    tracing::info!("Loading configuration..");

    tracing::info!("Connecting to database..");

    tracing::info!("Launching core modules..");

    tracing::info!("Loyalty Engine shutting down..");

    Ok(())

}
