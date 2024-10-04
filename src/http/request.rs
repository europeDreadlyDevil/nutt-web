use crate::http::method::Method;
use crate::http::HttpBody;
use crate::modules::session::Session;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use serde_json::Error;
use std::any::Any;
use std::collections::HashMap;
use std::sync::{Arc, RwLock, RwLockReadGuard};

#[derive(Debug)]
pub struct Request {
    method: Method,
    session: Arc<Option<Session>>,
    states: Arc<RwLock<HashMap<String, Box<dyn Any + Send + Sync>>>>,
    body: HttpBody,
}

impl Request {
    pub(crate) fn set_session(&mut self, session: Arc<Option<Session>>) {
        self.session = session;
    }
}

impl Request {
    pub(crate) fn set_states(
        &mut self,
        states: Arc<RwLock<HashMap<String, Box<dyn Any + Send + Sync>>>>,
    ) {
        self.states = states;
    }

    pub fn get_method(&self) -> Method {
        self.method.clone()
    }
}

impl Request {
    pub fn body_json<T: for<'a> Deserialize<'a> + DeserializeOwned>(&self) -> Result<T, Error> {
        serde_json::from_str::<T>(self.body.body.as_str().unwrap())
    }

    pub fn get_state(&self) -> RwLockReadGuard<'_, HashMap<String, Box<dyn Any + Send + Sync>>> {
        self.states.try_read().unwrap()
    }

    pub fn get_session(&self) -> Arc<Option<Session>> {
        self.session.clone()
    }
}

pub struct RequestBuilder {
    method: Method,
    body: HttpBody,
    states: HashMap<String, Box<dyn Any + Send + Sync>>,
    session: Option<Session>,
}

impl RequestBuilder {
    pub fn new<T: Serialize + Clone + Send>(method: Method, body: T) -> Self {
        Self {
            method,
            body: HttpBody::new(serde_json::to_value(body).unwrap()),
            states: HashMap::new(),
            session: None,
        }
    }

    pub fn build(self) -> Request {
        Request {
            method: self.method,
            body: self.body,
            states: Arc::new(RwLock::new(self.states)),
            session: Arc::new(None),
        }
    }
}
