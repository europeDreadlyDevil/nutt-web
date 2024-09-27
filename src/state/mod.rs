use std::sync::{Arc, RwLock};

pub struct State<T>(Arc<RwLock<T>>);



impl<T> State<T> {
    pub fn new(value: T) -> Self {
        Self(Arc::new(RwLock::new(value)))
    }
}