use std::any::Any;
use std::collections::HashMap;
use std::fmt::Debug;
use std::sync::{Arc, RwLock, RwLockReadGuard};
use serde::{Deserialize, Serialize};
use serde::de::DeserializeOwned;
use serde_json::Error;
use crate::http::HttpBody;
use crate::http::method::Method;

pub struct Request {
    method: Method,
    states: Arc<RwLock<HashMap<String, Box<dyn Any + Send + Sync>>>>,
    body: HttpBody
}

impl Request {
    pub(crate) fn set_states(&mut self, states: Arc<RwLock<HashMap<String, Box<dyn Any + Send + Sync>>>>) {
        self.states = states;
    }
}

impl Request {
    pub fn body_json<T: for<'a> Deserialize<'a> + DeserializeOwned>(&self) -> Result<T, Error> {
        serde_json::from_str::<T>(self.body.body.as_str().unwrap())
    }

    pub fn get_state(&self) -> RwLockReadGuard<'_, HashMap<String, Box<dyn Any + Send + Sync>>> {
         self.states.try_read().unwrap()
    }
}

pub struct RequestBuilder {
    method: Method,
    body: HttpBody,
    states: HashMap<String, Box<dyn Any + Send + Sync>>
}

impl RequestBuilder {
    pub fn new<T: Serialize + Clone + Send>(method: Method, request: T) -> Self {
        Self {
            method,
            body: HttpBody::new(serde_json::to_value(request).unwrap()),
            states: HashMap::new()
        }
    }

    pub fn build(self) -> Request {
        Request {
            method: self.method,
            body: self.body,
            states: Arc::new(RwLock::new(self.states))
        }
    }
}