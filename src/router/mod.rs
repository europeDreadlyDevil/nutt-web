use crate::http::method::Method;
use crate::router::route::Route;
use std::collections::HashMap;

pub mod route;

pub use nutt_web_macro::{delete, get, post, put};

pub struct Router {
    routes: HashMap<(Method, String), Route>,
}

impl Router {
    pub fn new() -> Self {
        Self {
            routes: HashMap::new(),
        }
    }

    pub fn insert(&mut self, key: (Method, String), route: Route) {
        self.routes.insert(key, route);
    }

    pub fn get(&self, key: (Method, String)) -> Option<&Route> {
        self.routes.get(&key)
    }
}

#[macro_export]
macro_rules! routes {
    ($elem:expr; $n:expr) => (
        vec![($elem)()]
    );
    ($($x:expr),+ $(,)?) => (
        Vec::from(vec![$(($x)()),+])
    );
    () => (
        Vec::new()
    )
 }
