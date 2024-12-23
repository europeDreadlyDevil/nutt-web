pub mod responder;

use crate::http::response::responder::Responder;
use crate::http::status::StatusCode;
use crate::http::{HttpBody, HttpHeader};
use serde::Serialize;
use serde_json::json;
use std::fmt::Display;

pub struct Response {
    header: HttpHeader,
    status: StatusCode,
    body: HttpBody,
}

impl Display for Response {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let resp = format!(
            "HTTP/1.1 {}\r\n{}\r\n{}\r\n\r\n",
            self.status,
            self.header,
            json!(self.body.body)
        );
        write!(f, "{}", resp)
    }
}
pub struct ResponseBuilder {
    status: StatusCode,
    header: HttpHeader,
    body: HttpBody,
}

impl ResponseBuilder {
    pub fn set_cookie(mut self, key: &str, item: String) -> Self {
        self.header
            .headers
            .insert("Set-Cookie".to_string(), format!("{}={};", key, item));
        self
    }
}

impl ResponseBuilder {
    pub fn new<T: Serialize + Clone + Send>(status_code: StatusCode, response: T) -> Self {
        Self {
            status: status_code,
            header: HttpHeader::new(response.clone()),
            body: HttpBody::new(serde_json::to_value(response).unwrap()),
        }
    }

    pub fn build(self) -> Response {
        Response {
            status: self.status,
            header: self.header,
            body: self.body,
        }
    }
}

impl Responder for Response {
    fn into_response(self) -> Response {
        self
    }
}
