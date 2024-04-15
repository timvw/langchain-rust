use async_trait::async_trait;
use std::error::Error;
use std::sync::Arc;

use tokio::sync::Mutex;

use crate::schemas::{memory::BaseMemory, messages::Message};

pub struct DummyMemroy {}

impl DummyMemroy {
    pub fn new() -> Self {
        Self {}
    }
}

impl Into<Arc<dyn BaseMemory>> for DummyMemroy {
    fn into(self) -> Arc<dyn BaseMemory> {
        Arc::new(self)
    }
}

impl Into<Arc<Mutex<dyn BaseMemory>>> for DummyMemroy {
    fn into(self) -> Arc<Mutex<dyn BaseMemory>> {
        Arc::new(Mutex::new(self))
    }
}

#[async_trait]
impl BaseMemory for DummyMemroy {
    async fn messages(&self) -> Result<Vec<Message>, Box<dyn Error>> {
        Ok(vec![])
    }

    async fn add_message(&mut self, _: Message) -> Result<(), Box<dyn Error>> {
        Ok(())
    }

    async fn clear(&mut self) -> Result<(), Box<dyn Error>> {
        Ok(())
    }
}
