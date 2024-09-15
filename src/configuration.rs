use config::{Config, File};
use serde::Deserialize;
#[derive(Debug, Deserialize)]
pub struct Configuration {
    pub app: AppSettings,
    pub database: DatabaseSettings,
}

#[derive(Debug, Deserialize)]
pub struct AppSettings {
    pub port: u16,
    pub host: String,
}

#[derive(Debug, Deserialize)]
pub struct DatabaseSettings {
    pub username: String,
    pub password: String,
    pub host: String,
    pub port: u16,
    pub name: String,
}

impl DatabaseSettings {
    pub fn connection_string(&self) -> String {
        format!(
            "mysql://{}:{}@{}:{}/{}",
            self.username, self.password, self.host, self.port, self.name
        )
    }
}

pub fn get_configuration() -> Result<Configuration, config::ConfigError> {
    // Initialise our configuration reader using ConfigBuilder
    let settings = Config::builder()
        .add_source(File::with_name("configuration"))
        .build()?;

    // Try to convert the configuration values it read into
    // our Settings type
    settings.try_deserialize()
}
