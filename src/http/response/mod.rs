pub mod responder;

use std::fmt::Display;
use serde::Serialize;
use serde_json::json;
use crate::http::{HttpBody, HttpHeader};
use crate::http::response::responder::Responder;
use crate::http::status::StatusCode;

pub struct Response<T:  Serialize + Clone + Send> {
    header: HttpHeader,
    status: StatusCode,
    body: HttpBody<T>
}

impl<T: Serialize + Clone + Send> Display for Response<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let resp = format!("HTTP/1.1 {}\r\n{}\r\n{}\r\n\r\n", self.status.to_string(), self.header.to_string(), json!(self.body.body).to_string());
        write!(f, "{}", resp)
    }
}
pub struct ResponseBuilder<T:  Serialize + Clone + Send> {
    status: StatusCode,
    header: HttpHeader,
    body: HttpBody<T>,
}

impl<T:  Serialize + Clone + Send> ResponseBuilder<T> {
    pub fn new(status_code: StatusCode, response: T) -> Self {
        Self {
            status: status_code,
            header: HttpHeader::new(response.clone()),
            body: HttpBody::new(response)
        }
    }

    pub fn build(self) -> Response<T> {
        Response {
            status: self.status,
            header: self.header,
            body: self.body
        }
    }
}

impl<T:  Serialize + Clone + Send> Responder<T> for Response<T> {
    fn into_response(self) -> Response<T> {
        self
    }
}