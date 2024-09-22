pub mod http;
pub mod router;

use std::error::Error;
use std::fmt::{Display, Formatter};
use std::future::Future;
use std::net::{SocketAddr, TcpStream};
use std::sync::Arc;
use serde::Serialize;
use serde_json::json;
use tokio::io::{AsyncBufReadExt, BufReader};
use tracing_log::log::{log, Level};
use crate::http::method::Method;
use crate::http::response::responder::Responder;
use crate::http::response::ResponseBuilder;
use crate::http::status::StatusCode;
use crate::router::route::Route;
use crate::router::Router;

pub struct NuttServer<T, F, Fut, R>
where T: Serialize + Clone + Send + Sync + 'static,
      F: Fn() -> Fut + Send + Sync + 'static,
      Fut: Future<Output=R> + Send + Sync + 'static,
      R: Responder<T> + Send + 'static
{
    address: Option<(String, u16)>,
    router: Router<T, F, Fut, R>
}

impl<T, F, Fut, R> NuttServer<T, F, Fut, R>
where T: Serialize + Clone + Send + Sync + 'static,
      F: Fn() -> Fut + Send + Sync + 'static,
      Fut: Future<Output=R> + Send + Sync + 'static,
      R: Responder<T> + Send + 'static
{
    pub fn new() -> Self {
        Self {
            address: None,
            router: Router::<T, F, Fut, R>::new()
        }
    }

    pub fn routes(mut self, routes: Vec<Route<T, F, Fut, R>>) -> Self {
        for route in routes {
            self.router.insert(&route.get_path(), route)
        }
        self
    }

    pub fn bind(mut self, address: (&str, u16)) -> Self {
        self.address = Some((address.0.to_string(), address.1));
        self
    }

    pub async fn run(mut self) {
        tracing_subscriber::fmt::init();
        if let Some(address) = self.address {
            let listener = tokio::net::TcpListener::bind(format!("{}:{}", address.0, address.1)).await.unwrap();
            log!(Level::Info, "Server started on {}:{}", address.0, address.1);
            let router = Arc::new(self.router);
            loop {
                let router_arc = router.clone();
                match listener.accept().await {
                    Ok((stream, _)) => {
                        tokio::spawn(async move {
                            match Self::handle_stream(stream).await {
                                Ok((method, path, stream)) => {
                                    if let Some(route) = router_arc.get_by_path(&path) {
                                        route.run_fabric(stream)
                                    }
                                }
                                Err(e) => log!(Level::Error, "Error handling stream: {}", e),
                            }
                        });
                    }
                    Err(e) => {
                        log!(Level::Error, "Failed to accept connection: {}", e);
                    }
                }
            }


        }
        else { panic!("Server don't have address") }
    }

    async fn handle_stream(mut stream: tokio::net::TcpStream) -> Result<(Method, String,  tokio::net::TcpStream), Box<dyn std::error::Error>> {
        let mut buf_reader = BufReader::new(&mut stream);
        let mut http_request = DisplayableVec(Vec::new());

        loop {
            let mut line = String::new();
            let bytes_read = buf_reader.read_line(&mut line).await?;
            if bytes_read == 0 || line == "\r\n" {
                break;
            }
            http_request.0.push(line);
        }

        log!(Level::Info, "Request: {http_request}");

        Ok((Method::GET, "/".to_string(), stream))
    }

}

pub struct DisplayableVec<T>(pub Vec<T>);

impl<T> Display for DisplayableVec<T>
where T: Display{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut out = String::new();
        for i in 0..self.0.len()-1{
            out.push_str(self.0[i].to_string().replace("\r", "").replace("\n", "").as_str());
            out.push_str("; ")
        }
        out.push_str(self.0[self.0.len()-1].to_string().as_str());

        write!(f, "{out}")
    }
}

