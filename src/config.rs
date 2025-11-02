use secrecy::{ExposeSecret, Secret};
use serde::Deserialize;

#[derive(Deserialize,Debug)]
pub struct Config{
    pub database_url : Secret<String>,
}


pub fn load_config() -> Result<Config, config::ConfigError>{
    dotenvy::dotenv().ok();

    let config = config::Config::builder()
    .add_source(config::Environment::default())
    .build()?
    .try_deserialize::<Config>()?;
    Ok(config)
}