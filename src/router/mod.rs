use std::collections::HashMap;
use std::future::Future;
use serde::Serialize;
use crate::http::response::responder::Responder;
use crate::router::route::Route;

pub mod route;

pub struct Router<T, F, Fut, R>
where T: Serialize + Clone + Send + Sync + 'static,
      F: Fn() -> Fut + Send + Sync + 'static,
      Fut: Future<Output=R> + Send + Sync + 'static,
      R: Responder<T> + Send + 'static
{
    routes: HashMap<String, Route<T, F, Fut, R>>
}

impl<T, F, Fut, R> Router<T, F, Fut, R>
where T: Serialize + Clone + Send + Sync + 'static,
      F: Fn() -> Fut + Send + Sync + 'static,
      Fut: Future<Output=R> + Send + Sync + 'static,
      R: Responder<T> + Send + 'static
{
    pub fn new() -> Self {
        Self {
            routes: HashMap::new()
        }
    }

    pub fn insert(&mut self, name: &str, route: Route<T, F, Fut, R>) {
        self.routes.insert(name.to_string(), route);
    }

    pub fn get_by_path(&self, path: &str) -> Option<&Route<T, F, Fut, R>> {
        self.routes.get(path)
    }
}