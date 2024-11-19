use anyhow::Result;
use async_trait::async_trait;
use crate::config::ApiConfig;
use crate::messages::Message;
use std::collections::HashMap;
use std::sync::Arc;

// First, declare the module
mod example_module;
// Re-export the module's public items
pub use example_module::ExampleModule;

// Trait that all modules must implement
#[async_trait]
pub trait Module: Send + Sync {
    // Self-initializing constructor that takes shared config
    fn new(config: Arc<ApiConfig>) -> Result<Self> where Self: Sized;
    async fn execute(&self) -> Result<Option<Message>>;
    fn name(&self) -> &'static str;
}

pub struct ModuleRegistry {
    modules: HashMap<&'static str, Box<dyn Module>>,
    config: Arc<ApiConfig>,
}

impl ModuleRegistry {
    pub fn new(config: Arc<ApiConfig>) -> Self {
        Self {
            modules: HashMap::new(),
            config,
        }
    }

    pub async fn register_modules(&mut self) -> Result<()> {
        self.register_module(Box::new(ExampleModule::new(self.config.clone())?));
        Ok(())
    }

    pub fn register_module(&mut self, module: Box<dyn Module>) -> Result<()> {
        let name = module.name();
        self.modules.insert(name, module);
        Ok(())
    }

    pub fn get_modules(&self) -> &HashMap<&'static str, Box<dyn Module>> {
        &self.modules
    }
}