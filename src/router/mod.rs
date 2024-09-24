use std::collections::HashMap;
use crate::http::method::Method;
use crate::router::route::Route;

pub mod route;

pub struct Router{
    routes: HashMap<(Method, String), Route>
}

impl Router{
    pub fn new() -> Self {
        Self {
            routes: HashMap::new()
        }
    }

    pub fn insert(&mut self, key: (Method, String), route: Route) {
        self.routes.insert(key, route);
    }

    pub fn get(&self, key: (Method, String)) -> Option<&Route> {
        self.routes.get(&key)
    }
}

#[macro_export] macro_rules! routes {
    ($elem:expr; $n:expr) => (
        vec![$elem]
    );
    ($($x:expr),+ $(,)?) => (
        Vec::from(vec![$($x),+])
    );
 }

#[macro_export]
macro_rules! box_route {
    // Макрос принимает функцию с аргументами
    ($func:expr, $($arg:ty),*) => {
        {
            use std::pin::Pin;
            use std::future::Future;
            use nutt_web::http::response::Response;
            use nutt_web::http::status::StatusCode;
            use nutt_web::http::response::ResponseBuilder;
            use serde::de::DeserializeOwned;
            use nutt_web::http::request::Request;

            move |req: Request| -> Pin<Box<dyn Future<Output = Response> + Send + Sync>> {

                $(
                    let arg: $arg = match req.body_json::<$arg>() {
                        Ok(data) => data,
                        Err(_) => return Box::pin( async { ResponseBuilder::new(StatusCode::BadRequest, "Invalid data").build() }),
                    };
                )*

                Box::pin($func(arg))
            } as fn(Request) -> _
        }
    };

    ($func:expr) => {
        {
            use std::pin::Pin;
            use std::future::Future;
            use nutt_web::http::response::Response;
            use nutt_web::http::request::Request;
            |req: Request| -> Pin<Box<dyn Future<Output = Response> + Send + Sync>> {
                Box::pin($func())
            } as fn(Request) -> _
        }
    };
}


