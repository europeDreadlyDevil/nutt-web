use crate::http::response::{Response, ResponseBuilder};
use crate::http::status::StatusCode;

pub trait Responder {
    fn into_response(self) -> Response;
}

impl Responder for String {
    fn into_response(self) -> Response {
        ResponseBuilder::new(StatusCode::Ok, self).build()
    }
}

impl Responder for i32 {
    fn into_response(self) -> Response {
        ResponseBuilder::new(StatusCode::Ok, self).build()
    }
}