use std::collections::HashMap;
use crate::http::method::Method;
use crate::router::route::Route;

pub mod route;

pub use nutt_web_macro::{get, post};

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
        vec![($elem)()]
    );
    ($($x:expr),+ $(,)?) => (
        Vec::from(vec![$(($x)()),+])
    );
    () => (
        Vec::new()
    )
 }

macro_rules! box_route {
    // Макрос для функций с любыми аргументами (state, body или их комбинация)
    ($func:expr, $( $arg:ident : $arg_ty:ty ),* ) => {
        {
            use std::pin::Pin;
            use std::future::Future;
            use nutt_web::http::response::Response;
            use nutt_web::http::request::Request;

            use serde::de::DeserializeOwned;

            move |req: Request, state: StateArgs| -> Pin<Box<dyn Future<Output = Response> + Send + Sync>> {
                let mut extracted_args = vec![];

                // Проходим по каждому аргументу и проверяем его тип
                $(
                    if std::any::TypeId::of::<$arg_ty>() == std::any::TypeId::of::<State<Any>>
                    // Если аргумент - тело JSON, десериализуем его
                    if std::any::TypeId::of::<$arg_ty>() == std::any::TypeId::of::<Json>() {
                        let body: $arg_ty = match req.body_json::<$arg_ty>().await {
                            Ok(data) => data,
                            Err(_) => return Box::pin(async { Response::bad_request("Invalid JSON body").into_response() }),
                        };
                        extracted_args.push(body);
                    }

                )*

                // Вызов функции с динамически собранными аргументами
                Box::pin($func(extracted_args))
            } as fn(Request, StateArgs) -> _
        }
    };

    // Макрос для функций без аргументов
    ($func:expr) => {
        {
            use std::pin::Pin;
            use std::future::Future;
            use nutt_web::http::response::Response;

            || -> Pin<Box<dyn Future<Output = Response> + Send + Sync>> {
                Box::pin($func())
            } as fn(Request, Args) -> _
        }
    };
}



