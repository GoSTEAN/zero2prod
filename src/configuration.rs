use secrecy::{Secret, ExposeSecret};
use dotenv::dotenv;
use std::env;
use config::{Config, ConfigError};

#[derive(serde::Deserialize)]
pub struct DatabaseSettings {
    pub username: String,
    pub password: Secret<String>,
    pub port: u16,
    pub host: String,
    pub database_name: String,
}

#[derive(serde::Deserialize)]
pub struct Settings {
    pub database: DatabaseSettings,
    pub application_port: u16
}

pub fn get_configuration() -> Result<Settings, ConfigError> {
    dotenv().ok();
    // Note: env::var("DATABASE_URL") result is currently unused
    
    // Initialize our configuration reader
    let settings = Config::builder()
        // Add configuration values from a file named `configuration.yaml`
        .add_source(config::File::with_name("configuration"))
        .build()?;
    
    // Try to convert the configuration values it read into our Settings type
    settings.try_deserialize::<Settings>()
}

impl DatabaseSettings {
    pub fn connection_string(&self) -> Secret<String>{
        Secret::new( format!(
            "postgres://{}:{}@{}:{}/{}",
            self.username,
            self.password.expose_secret(),
            self.host,
            self.port,
            self.database_name
        ))
    }

    pub fn connection_string_without_db(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}",
            self.username,
            self.password.expose_secret(),
            self.host,
            self.port
        )
    }
}

