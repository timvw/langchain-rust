use super::messages::Message;
use async_trait::async_trait;
use std::error::Error;

#[async_trait]
pub trait BaseMemory: Send + Sync {
    async fn messages(&self) -> Result<Vec<Message>, Box<dyn Error>>;

    // Use a trait object for Display instead of a generic type
    async fn add_user_message(&mut self, message: &str) -> Result<(), Box<dyn Error>> {
        // Convert the Display trait object to a String and pass it to the constructor
        let result = self
            .add_message(Message::new_human_message(message))
            .await?;
        Ok(result)
    }

    // Use a trait object for Display instead of a generic type
    async fn add_ai_message(&mut self, message: &str) -> Result<(), Box<dyn Error>> {
        // Convert the Display trait object to a String and pass it to the constructor
        let result = self.add_message(Message::new_ai_message(message)).await?;
        Ok(result)
    }

    async fn add_message(&mut self, message: Message) -> Result<(), Box<dyn Error>>;

    async fn clear(&mut self) -> Result<(), Box<dyn Error>>;

    async fn to_string(&self) -> Result<String, Box<dyn Error>> {
        let result = self
            .messages()
            .await?
            .iter()
            .map(|msg| format!("{}: {}", msg.message_type.to_string(), msg.content))
            .collect::<Vec<String>>()
            .join("\n");
        Ok(result)
    }
}

impl<M> From<M> for Box<dyn BaseMemory>
where
    M: BaseMemory + 'static,
{
    fn from(memory: M) -> Self {
        Box::new(memory)
    }
}
