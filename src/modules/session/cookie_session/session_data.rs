use std::any::Any;
use std::collections::HashMap;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct Data {
    data: HashMap<String, Arc<Box<dyn Any + Send + Sync>>>,
}

impl Data {
    pub fn new() -> Self {
        Self {
            data: HashMap::new(),
        }
    }

    pub fn get<T: Send + Sync + 'static>(&self, id: &str) -> Option<&T> {
        if let Some(val) = self.data.get(id) {
            return val.downcast_ref::<T>();
        }
        None
    }

    pub fn set<T: Send + Sync + 'static>(&mut self, id: &str, item: T) {
        self.data.insert(id.to_string(), Arc::new(Box::new(item)));
    }
}
