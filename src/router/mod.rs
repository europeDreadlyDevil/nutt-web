use std::collections::HashMap;
use std::future::Future;
use crate::http::method::Method;
use crate::http::response::responder::Responder;
use crate::router::route::Route;

pub mod route;

pub struct Router<F, Fut, R>
where F: Fn() -> Fut + Send + Sync + 'static,
      Fut: Future<Output=R> + Send + Sync + 'static,
      R: Responder + Send + 'static
{
    routes: HashMap<(Method, String), Route<F, Fut, R>>
}

impl<F, Fut, R> Router<F, Fut, R>
where F: Fn() -> Fut + Send + Sync + 'static,
      Fut: Future<Output=R> + Send + Sync + 'static,
      R: Responder + Send + 'static
{
    pub fn new() -> Self {
        Self {
            routes: HashMap::new()
        }
    }

    pub fn insert(&mut self, key: (Method, String), route: Route<F, Fut, R>) {
        self.routes.insert(key, route);
    }

    pub fn get(&self, key: (Method, String)) -> Option<&Route<F, Fut, R>> {
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

#[macro_export] macro_rules! box_route {
     ($func:expr) => {
         {
             use std::pin::Pin;
             use std::future::Future;
             use nutt_web::http::response::Response;

             || -> Pin<Box<dyn Future<Output = Response> + Send + Sync>> {
                 Box::pin($func())
             } as fn() -> _
         }

     };
 }

