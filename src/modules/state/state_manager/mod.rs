pub mod state_manager_message;

use std::any::Any;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use tokio::sync::mpsc::UnboundedReceiver;
use state_manager_message::StateManagerMessage;
use anyhow::Result;

pub struct StateManager {
    unbounded_receiver: UnboundedReceiver<StateManagerMessage<Box<dyn Any + Send + Sync>>>,
    states: Arc<RwLock<HashMap<String, Box<dyn Any + Send + Sync>>>>,
}

impl StateManager {
    pub async fn listen(mut self) -> Result<()> {
        loop {
            if let Some(message) = self.unbounded_receiver.recv().await {
                match message {
                    StateManagerMessage::GET { .. } => {}
                    StateManagerMessage::POST { .. } => {}
                    StateManagerMessage::PUT { .. } => {}
                    StateManagerMessage::DELETE { .. } => {}
                }
            }
        }
    }
}