use anyhow::Result;
use config::{Config, Environment};
use std::sync::Arc;

#[derive(Clone)]
pub struct ApiConfig {
    pub environment: String,
    pub log_level: String,
}

impl ApiConfig {
    pub fn new() -> Result<Arc<Self>> {
        dotenv::dotenv().ok();

        let config = Config::builder()
            // Default values
            .set_default("environment", "development")?
            .set_default("log_level", "info")?
            // Load from environment variables
            .add_source(Environment::default())
            .build()?;

        Ok(Arc::new(Self {
            environment: config.get_string("environment")?,
            log_level: config.get_string("log_level")?,
        }))
    }
} 