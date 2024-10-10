use crate::http::method::Method;
use crate::modules::router::route::Route;
use std::collections::HashMap;
pub mod route;

pub struct Router {
    routes: HashMap<(Method, String), Route>,
}

impl Default for Router {
    fn default() -> Self {
        Self::new()
    }
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
