use serde::{Deserialize, Serialize};
use serde::de::DeserializeOwned;
use serde_json::Error;
use crate::http::HttpBody;
use crate::http::method::Method;

pub struct Request {
    method: Method,
    body: HttpBody
}

impl Request {
    pub fn body_json<T: for<'a> Deserialize<'a> + DeserializeOwned>(&self) -> Result<T, Error> {
        println!("{}", self.body.body);
        serde_json::from_str::<T>(self.body.body.as_str().unwrap())
    }
}

pub struct RequestBuilder {
    method: Method,
    body: HttpBody
}

impl RequestBuilder {
    pub fn new<T: Serialize + Clone + Send>(method: Method, request: T) -> Self {
        Self {
            method,
            body: HttpBody::new(serde_json::to_value(request).unwrap())
        }
    }

    pub fn build(self) -> Request {
        Request {
            method: self.method,
            body: self.body
        }
    }
}