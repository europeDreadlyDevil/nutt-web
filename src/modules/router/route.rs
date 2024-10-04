use crate::http::method::Method;
use crate::http::request::Request;
use crate::http::response::responder::Responder;
use crate::http::response::Response;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use tokio::net::TcpStream;

type FuncPointer = fn(Request) -> Pin<Box<dyn Future<Output = Response> + Send + Sync>>;
pub struct Route {
    method: Method,
    path: String,
    fabric: Arc<FuncPointer>,
}

impl Route {
    pub fn run_fabric(&self, stream: TcpStream, req: Request) {
        println!("{:?}", req);
        let fabric = self.fabric.clone();
        tokio::spawn(async move {
            let resp = fabric(req).await;
            stream
                .try_write(resp.into_response().to_string().as_bytes())
                .unwrap()
        });
    }
}

impl Route {
    pub fn new(method: Method, path: &str, fabric: FuncPointer) -> Self {
        Self {
            method,
            path: path.to_string(),
            fabric: Arc::new(fabric),
        }
    }

    #[inline]
    pub fn get(&self) -> (Method, String) {
        (self.method.clone(), self.path.clone())
    }
}
