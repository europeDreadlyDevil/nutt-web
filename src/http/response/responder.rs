use serde::Serialize;
use crate::http::response::{Response, ResponseBuilder};
use crate::http::status::StatusCode;

pub trait Responder<T: Serialize + Clone + Send> {
    fn into_response(self) -> Response<T>;
}

impl Responder<String> for String {
    fn into_response(self) -> Response<String> {
        ResponseBuilder::new(StatusCode::Ok, self).build()
    }
}