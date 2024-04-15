use async_trait::async_trait;
use std::error::Error;
use std::sync::Arc;

use tokio::sync::Mutex;

use crate::schemas::{memory::BaseMemory, messages::Message};

pub struct WindowBufferMemory {
    window_size: usize,
    messages: Vec<Message>,
}

impl Default for WindowBufferMemory {
    fn default() -> Self {
        Self::new(10)
    }
}

impl WindowBufferMemory {
    pub fn new(window_size: usize) -> Self {
        Self {
            messages: Vec::new(),
            window_size,
        }
    }
}

impl Into<Arc<dyn BaseMemory>> for WindowBufferMemory {
    fn into(self) -> Arc<dyn BaseMemory> {
        Arc::new(self)
    }
}

impl Into<Arc<Mutex<dyn BaseMemory>>> for WindowBufferMemory {
    fn into(self) -> Arc<Mutex<dyn BaseMemory>> {
        Arc::new(Mutex::new(self))
    }
}

#[async_trait]
impl BaseMemory for WindowBufferMemory {
    async fn messages(&self) -> Result<Vec<Message>, Box<dyn Error>> {
        Ok(self.messages.clone())
    }

    async fn add_message(&mut self, message: Message) -> Result<(), Box<dyn Error>> {
        if self.messages.len() >= self.window_size {
            self.messages.remove(0);
        }
        self.messages.push(message);
        Ok(())
    }

    async fn clear(&mut self) -> Result<(), Box<dyn Error>> {
        self.messages.clear();
        Ok(())
    }
}
