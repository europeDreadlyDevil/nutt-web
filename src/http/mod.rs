use serde::Serialize;
use serde_json::{json, Value};
use std::fmt::{Display, Formatter};

pub mod method;
pub mod request;
pub mod response;
pub mod status;

pub struct HttpHeader {
    headers: Value,
}

impl HttpHeader {
    pub fn new<T: Serialize + Clone + Send>(response: T) -> Self {
        let content = serde_json::to_string(&response).unwrap();
        let base_headers = json!({
            "Content-Type": "application/json",
            "Content-Length": content.len(),
            "Connection": "keep-alive"
        });

        Self {
            headers: base_headers,
        }
    }
}

impl Display for HttpHeader {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut headers = String::new();
        for (key, value) in self.headers.as_object().unwrap() {
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
