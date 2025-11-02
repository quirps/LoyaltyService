
mod config;
mod database;

#[tokio::main]

async fn main() ->  Result<(), Box<dyn std::error::Error>> {
    //initialize debugging
    tracing_subscriber::fmt().with_env_filter(tracing_subscriber::EnvFilter::from_default_env()).init();

    tracing::info!("Loyalty Engine Starting!");

    tracing::info!("Loading configuration..");
    
    let config = config::load_config()?; 
    tracing::info!("Connecting to database..");
    let db_pool = database::create_pool(&config.database_url);
    tracing::info!("Launching core modules..");

    tracing::info!("Loyalty Engine shutting down..");

    Ok(())

}
