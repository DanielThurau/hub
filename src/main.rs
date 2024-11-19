use anyhow::Result;
use messages::{Message, MessageType};
use std::collections::{HashMap, VecDeque};
use std::sync::Arc;
use tokio::sync::RwLock;
use tokio::time::{Duration, interval};

mod modules;
mod config;
mod messages;
use modules::ModuleRegistry;
use config::ApiConfig;

struct MainBus {
    registry: Arc<RwLock<ModuleRegistry>>,
    tick_rate: Duration,
    config: Arc<ApiConfig>,
    queues: HashMap<MessageType, VecDeque<Message>>,
}

impl MainBus {
    async fn new(tick_rate: Duration) -> Result<Self> {
        let config = ApiConfig::new()?;

        let mut queues = HashMap::new();
        queues.insert(MessageType::Twilio, VecDeque::new());
        queues.insert(MessageType::App, VecDeque::new());
        
        Ok(Self {
            registry: Arc::new(RwLock::new(ModuleRegistry::new(config.clone()))),
            tick_rate,
            config,
            queues,
        })
    }

    async fn startup(&self) -> Result<()> {
        let mut registry = self.registry.write().await;
        registry.register_modules().await?;
        Ok(())
    }

    async fn run(&mut self) -> Result<()> {
        let mut interval = interval(self.tick_rate);

        loop {
            interval.tick().await;
            
            let registry = self.registry.read().await;
            
            for module in registry.get_modules().values() {
                if let Ok(Some(message)) = module.execute().await {
                    let queue_type = message.get_queue_type();
                    if let Some(queue) = self.queues.get_mut(&queue_type) {
                        queue.push_back(message);
                    }
                }
            }

            // Process messages in queues here
            if let Some(twilio_queue) = self.queues.get_mut(&MessageType::Twilio) {
                while let Some(message) = twilio_queue.pop_front() {
                    if let Message::Twilio(twilio_msg) = message {
                        tracing::info!("Processing Twilio message: {}", twilio_msg.message);
                    }
                }
            }
        }
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    tracing_subscriber::fmt::init();

    // Create main bus with 100ms tick rate
    let mut main_bus = MainBus::new(Duration::from_millis(100)).await?;
    
    // Run startup sequence
    main_bus.startup().await?;
    
    // Start main loop
    main_bus.run().await?;

    Ok(())
}
