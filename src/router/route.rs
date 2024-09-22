use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use serde::Serialize;
use tokio::net::TcpStream;
use crate::http::response::responder::Responder;

pub struct Route<T, F, Fut, R>
where T: Serialize + Clone + Send + Sync + 'static,
      F: Fn() -> Fut + Send + Sync + 'static,
      Fut: Future<Output=R> + Send + Sync + 'static,
      R: Responder<T> + Send + 'static
{
    path: String,
    fabric: Arc<F>,
    ty: Option<T>
}

impl<T, F, Fut, R> Route<T, F, Fut, R>
where T: Serialize + Clone + Send + Sync + 'static,
      F: Fn() -> Fut + Send + Sync + 'static,
      Fut: Future<Output=R> + Send + Sync + 'static,
      R: Responder<T> + Send + 'static
{
    pub fn run_fabric(&self, stream: TcpStream) {
        let fabric = self.fabric.clone();
        tokio::spawn(async move {
            let resp = fabric().await;
            stream.try_write(resp.into_response().to_string().as_bytes()).unwrap()
        });
    }
}

impl<T, F, Fut, R> Route<T, F, Fut, R>
where T: Serialize + Clone + Send + Sync + 'static,
      F: Fn() -> Fut + Send + Sync + 'static,
      Fut: Future<Output=R> + Send + Sync + 'static,
      R: Responder<T> + Send + 'static
{
    pub fn new(path: &str, fabric: F) -> Self {
        Self { path: path.to_string(), fabric: Arc::new(fabric), ty: None}
    }

    #[inline]
    pub fn get_path(&self) -> String {
        self.path.clone()
    }
}

#[macro_export] macro_rules! get {
    ($path:expr, $fabric:expr ) => {
        {
            use nutt_web::router::route::Route;
            Route::new($path, $fabric)
        }
    };
}
