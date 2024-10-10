use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct NuttConfig {
    service: HashMap<String, ServiceConfig>,
}

impl NuttConfig {
    pub fn new() -> Self {
        Self {
            service: HashMap::new(),
        }
    }

    pub fn get_service_config(&self, name: &str) -> Option<ServiceConfig> {
        if let Some(conf) = self.service.get(name) {
            return Some(conf.clone());
        }
        None
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ServiceConfig {
    local_host: String,
    local_port: u16,
}

impl ServiceConfig {
    pub fn new(local_host: &str, local_port: u16) -> Self {
        Self {
            local_host: local_host.to_string(),
            local_port,
        }
    }

    pub fn get_addr(&self) -> (&str, u16) {
        (&self.local_host, self.local_port)
    }
}
