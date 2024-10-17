use std::any::Any;
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};
use anyhow::{Error, Result};
use crate::modules::state::state_manager::state_manager_message::StateManagerMessage;

pub struct StateHandler<T> {
    state_ident: String,
    sender: UnboundedSender<StateManagerMessage<T>>,
    receiver: UnboundedReceiver<Box<dyn Any + Send + Sync>>,
}

impl<T: Send + Sync + Sized + 'static> StateHandler<T> {
    pub async fn read(&mut self) -> Result<T> {
        self.sender.send(StateManagerMessage::GET {state_ident: self.state_ident.clone()})?;
        loop {
            if let Some(received_msg) = self.receiver.recv().await {
                if let Ok(value) = received_msg.downcast::<T>() {
                    return Ok(*value)
                }
                return Err(Error::msg("Error: can't downcast type"))
            }
        }
    }

    pub async fn write(&mut self, value: T) -> Result<()> {
        self.sender.send(StateManagerMessage::POST {state_ident: self.state_ident.clone(), value})?;
        Ok(())
    }
}