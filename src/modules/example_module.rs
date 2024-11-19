use super::Module;
use crate::config::ApiConfig;
use crate::messages::{Message, TwilioMessage};
use anyhow::Result;
use async_trait::async_trait;
use std::sync::Arc;

pub struct ExampleModule {
    name: &'static str,
    config: Arc<ApiConfig>,
}

#[async_trait]
impl Module for ExampleModule {
    fn new(config: Arc<ApiConfig>) -> Result<Self> {
        Ok(Self {
            name: "example_module",
            config,
        })
    }

    async fn execute(&self) -> Result<Option<Message>> {
        tracing::info!("Executing in {} environment", self.config.environment);
        
        // Example of returning a message
        Ok(Some(Message::Twilio(TwilioMessage {
            message: "Example message".to_string(),
        })))
    }

    fn name(&self) -> &'static str {
        self.name
    }
} 