use crate::error::app_error::AppError;
use serde::Deserialize;
use std::fs::File;

#[derive(Debug, Deserialize)]
pub struct AppConfig {
    pub server: ServerConfig,
    pub auth: AuthConfig,
}

#[derive(Debug, Deserialize)]
pub struct ServerConfig {
    pub port: u16,
    pub db_url: String,
}

#[derive(Debug, Deserialize)]
pub struct AuthConfig {
    pub ek: String,
    pub dk: String,
}

impl AppConfig {
    pub fn new() -> Result<Self, AppError> {
        let rdr = File::open("./backend.yaml")?;
        let config = serde_yaml::from_reader(rdr)?;
        Ok(config)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;

    #[test]
    fn app_config_should_work() -> Result<()> {
        let config = AppConfig::new()?;

        assert_eq!(config.server.port, 6688);
        assert_eq!(
            config.server.db_url,
            "postgres://postgres:postgres@localhost:5432/ferris"
        );
        assert_eq!(
            config.auth.ek,
            "-----BEGIN PRIVATE KEY-----\nMC4CAQAwBQYDK2VwBCIEIO86NLYAOor1kUohceuaT9susMROxY973ceRUg+LQx97\n-----END PRIVATE KEY-----\n"
        );
        assert_eq!(
            config.auth.dk,
            "-----BEGIN PUBLIC KEY-----\nMCowBQYDK2VwAyEAlCHtaGQUJ64HH7fP2rxuqkhoOl6mEYbNJbPuvAdao6I=\n-----END PUBLIC KEY-----\n"
        );

        Ok(())
    }
}
