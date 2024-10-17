use crate::http::cookie::CookieJar;
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
    cookie_jar: CookieJar,
}

impl Request {
    pub fn set_states(
        &mut self,
        states: Arc<RwLock<HashMap<String, Box<dyn Any + Send + Sync>>>>,
    ) {
        self.states = states;
    }
    pub fn set_session(&mut self, session: Arc<Option<Session>>) {
        self.session = session;
    }

    pub fn set_cookie_jar(&mut self, cookie_jar: CookieJar) {
        self.cookie_jar = cookie_jar
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
    pub fn get_method(&self) -> Method {
        self.method.clone()
    }

    pub fn get_cookie_jar(&self) -> CookieJar {
        self.cookie_jar.clone()
    }
}

pub struct RequestBuilder {
    method: Method,
    body: HttpBody,
    states: HashMap<String, Box<dyn Any + Send + Sync>>,
    session: Option<Session>,
    cookie_jar: CookieJar,
}

impl RequestBuilder {
    pub fn new<T: Serialize + Clone + Send>(method: Method, body: T) -> Self {
        Self {
            method,
            body: HttpBody::new(serde_json::to_value(body).unwrap()),
            states: HashMap::new(),
            session: None,
            cookie_jar: CookieJar::new(),
        }
    }

    pub(crate) fn set_cookie_jar(mut self, cookie_jar: CookieJar) -> Self {
        self.cookie_jar = cookie_jar;
        self
    }

    pub fn build(self) -> Request {
        Request {
            method: self.method,
            body: self.body,
            states: Arc::new(RwLock::new(self.states)),
            session: Arc::new(self.session),
            cookie_jar: self.cookie_jar,
        }
    }
}
