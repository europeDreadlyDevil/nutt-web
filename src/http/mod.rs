use serde::Serialize;
use serde_json::Value;
use std::collections::HashMap;
use std::fmt::{Display, Formatter};

pub mod cookie;
pub mod method;
pub mod request;
pub mod response;
pub mod status;

pub struct HttpHeader {
    headers: HashMap<String, String>,
}

impl HttpHeader {
    pub fn new<T: Serialize + Clone + Send>(response: T) -> Self {
        let content = serde_json::to_string(&response).unwrap();
        let mut base_headers = HashMap::new();
        base_headers.insert("Content-Type".to_string(), "application/json".to_string());
        base_headers.insert("Content-Length".to_string(), content.len().to_string());
        base_headers.insert("Connection".to_string(), "keep-alive".to_string());

        Self {
            headers: base_headers,
        }
    }
}

impl Display for HttpHeader {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut headers = String::new();
        for (key, value) in &self.headers {
            headers.push_str(&format!("{}: {}\r\n", key, value))
        }
        write!(f, "{}", headers)
    }
}

#[derive(Clone, Debug)]
pub struct HttpBody {
    body: Value,
}

impl HttpBody {
    pub fn new(value: Value) -> HttpBody {
        Self { body: value }
    }
}
