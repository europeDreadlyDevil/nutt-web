use std::fmt::{Display, Formatter};
use serde::Serialize;
use serde_json::{json, Value};

pub mod status;
pub mod response;
pub mod method;
pub mod request;

pub struct HttpHeader {
    headers: Value
}

impl HttpHeader {
    pub fn new<T:  Serialize + Clone + Send>(response: T) -> Self {
        let content = serde_json::to_string(&response).unwrap();
        let base_headers = json!({
            "Content-Type": "application/json",
            "Content-Length": content.len(),
            "Connection": "keep-alive"
        });

        Self {
            headers: base_headers
        }
    }
}

impl Display for HttpHeader {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut headers = String::new();
        for (key, value) in self.headers.as_object().unwrap() {
            headers.push_str(&format!("{}: {}\r\n", key, value.to_string()))
        }
        write!(f, "{}", headers)
    }
}

#[derive(Clone)]
pub struct HttpBody{
    body: Value
}

impl HttpBody {
    pub fn new(value: Value) -> HttpBody {
        Self {body: value }
    }
}
