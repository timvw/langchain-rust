use async_trait::async_trait;
use std::error::Error;
use std::sync::Arc;

use tokio::sync::Mutex;

use crate::schemas::{memory::BaseMemory, messages::Message};

pub struct SimpleMemory {
    messages: Vec<Message>,
}

impl SimpleMemory {
    pub fn new() -> Self {
        Self {
            messages: Vec::new(),
        }
    }
}

impl Into<Arc<dyn BaseMemory>> for SimpleMemory {
    fn into(self) -> Arc<dyn BaseMemory> {
        Arc::new(self)
    }
}

impl Into<Arc<Mutex<dyn BaseMemory>>> for SimpleMemory {
    fn into(self) -> Arc<Mutex<dyn BaseMemory>> {
        Arc::new(Mutex::new(self))
    }
}

#[async_trait]
impl BaseMemory for SimpleMemory {
    async fn messages(&self) -> Result<Vec<Message>, Box<dyn Error>> {
        Ok(self.messages.clone())
    }

    async fn add_message(&mut self, message: Message) -> Result<(), Box<dyn Error>> {
        self.messages.push(message);
        Ok(())
    }

    async fn clear(&mut self) -> Result<(), Box<dyn Error>> {
        self.messages.clear();
        Ok(())
    }
}
