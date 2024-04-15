use async_trait::async_trait;
use std::sync::Arc;

use tokio::sync::Mutex;

use crate::schemas::{memory::BaseMemory, messages::Message, MemoryError};

pub struct DummyMemory {}

impl DummyMemory {
    pub fn new() -> Self {
        Self {}
    }
}

impl Into<Arc<dyn BaseMemory>> for DummyMemory {
    fn into(self) -> Arc<dyn BaseMemory> {
        Arc::new(self)
    }
}

impl Into<Arc<Mutex<dyn BaseMemory>>> for DummyMemory {
    fn into(self) -> Arc<Mutex<dyn BaseMemory>> {
        Arc::new(Mutex::new(self))
    }
}

#[async_trait]
impl BaseMemory for DummyMemory {
    async fn messages(&self) -> Result<Vec<Message>, MemoryError> {
        Ok(vec![])
    }

    async fn add_message(&mut self, _: Message) -> Result<(), MemoryError> {
        Ok(())
    }

    async fn clear(&mut self) -> Result<(), MemoryError> {
        Ok(())
    }
}
