mod session_data;

use crate::modules::session::cookie_session::session_data::Data;
use base64ct::Encoding;
use serde::Serialize;
use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::ops::Deref;
use std::sync::{Arc, RwLock};
use whirlpool::{Digest, Whirlpool};

#[derive(Debug, Clone)]
pub struct CookieSession {
    sessions: Arc<RwLock<HashMap<SessionId, Data>>>,
}

impl CookieSession {
    pub fn set_data_by_id<T: Sync + Send + 'static>(&self, id: SessionId, item: (&str, T)) {
        if let Ok(mut session) = self.sessions.try_write() {
            if let Some(data) = session.get_mut(&id) {
                data.set(item.0, item.1)
            }
        }
    }
}

impl CookieSession {
    pub fn new() -> Self {
        Self {
            sessions: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub fn create_new_session(&mut self) -> SessionId {
        let id = SessionId::new();
        if let Ok(mut session) = self.sessions.clone().try_write() {
            session.insert(id.clone(), Data::new());
        };
        id
    }

    pub fn get_session_data(&self, id: SessionId) -> Option<Data> {
        if let Ok(session) = self.sessions.try_read() {
            return if let Some(data) = session.get(&id) {
                Some(data.clone())
            } else {
                None
            };
        }
        None
    }
}

#[derive(Hash, Clone, Eq, PartialEq, Debug)]
pub struct SessionId(String);

impl SessionId {
    pub fn new() -> Self {
        let mut hasher = Whirlpool::new();
        hasher.update(chrono::Utc::now().to_string().as_bytes());
        Self(base64ct::Base64::encode_string(&hasher.finalize()))
    }
}

impl Display for SessionId {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<String> for SessionId {
    fn from(value: String) -> Self {
        Self(value)
    }
}
