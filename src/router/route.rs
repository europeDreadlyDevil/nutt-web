use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use tokio::net::TcpStream;
use crate::http::method::Method;
use crate::http::response::responder::Responder;

pub struct Route<F, Fut, R>
where F: Fn() -> Fut + Send + Sync + 'static,
      Fut: Future<Output=R> + Send + Sync + 'static,
      R: Responder + Send + 'static
{
    method: Method,
    path: String,
    fabric: Arc<Pin<Box<F>>>,
}

impl<F, Fut, R> Route<F, Fut, R>
where F: Fn() -> Fut + Send + Sync + 'static,
      Fut: Future<Output=R> + Send + Sync + 'static,
      R: Responder + Send + 'static
{
    pub fn run_fabric(&self, stream: TcpStream) {
        let fabric = self.fabric.clone();
        tokio::spawn(async move {
            let resp = fabric().await;
            stream.try_write(resp.into_response().to_string().as_bytes()).unwrap()
        });
    }
}

impl<F, Fut, R> Route<F, Fut, R>
where F: Fn() -> Fut + Send + Sync + 'static,
      Fut: Future<Output=R> + Send + Sync + 'static,
      R: Responder + Send + 'static
{
    pub fn new(method: Method, path: &str, fabric: F) -> Self {
        Self { method, path: path.to_string(), fabric: Arc::new(Box::pin(fabric))}
    }

    #[inline]
    pub fn get(&self) -> (Method, String) {
        (self.method.clone(), self.path.clone())
    }
}

#[macro_export] macro_rules! get {
    ($path:expr, $fabric:expr ) => {
        {
            use nutt_web::http::method::Method;
            use nutt_web::router::route::Route;
            use std::future::Future;
            use std::pin::Pin;
            Route::new(Method::GET, $path, Box::pin($fabric()))
        }
    };
}
