use std::sync::{Arc, RwLock, RwLockReadGuard, RwLockWriteGuard};

#[derive(Debug, Clone)]
pub struct State<T>(Arc<RwLock<T>>);



impl<T> State<T> {
    pub fn new(value: T) -> Self {
        Self(Arc::new(RwLock::new(value)))
    }
    pub fn read(&self) -> RwLockReadGuard<T> {
        while let Err(_) = self.0.try_read() {}
        self.0.read().unwrap()
    }

    pub fn write(&self) -> RwLockWriteGuard<T> {
        while let Err(_) = self.0.try_write() {}
        self.0.write().unwrap()
    }

}

#[macro_export] macro_rules! state {
     ($state:expr) => {
         (stringify!($state).into(), $state)
     };
 }