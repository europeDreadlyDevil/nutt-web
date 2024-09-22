pub mod http;
pub mod router;

use std::error::Error;
use std::fmt::{Display, Formatter};
use std::future::Future;
use std::sync::Arc;
use tokio::io::{AsyncBufReadExt, BufReader};
use tracing_log::log::{log, Level};
use crate::http::method::Method;
use crate::http::response::responder::Responder;
use crate::router::route::Route;
use crate::router::Router;

pub struct NuttServer<F, Fut, R>
where F: Fn() -> Fut + Send + Sync + 'static,
      Fut: Future<Output=R> + Send + Sync + 'static,
      R: Responder + Send + 'static
{
    address: Option<(String, u16)>,
    router: Router<F, Fut, R>
}

impl<F, Fut, R> NuttServer<F, Fut, R>
where F: Fn() -> Fut + Send + Sync + 'static,
      Fut: Future<Output=R> + Send + Sync + 'static,
      R: Responder + Send + 'static
{
    pub fn new() -> Self {
        Self {
            address: None,
            router: Router::new()
        }
    }

    pub fn routes(mut self, routes: Vec<Route<F, Fut, R>>) -> Self {
        for route in routes {
            self.router.insert(route.get(), route)
        }
        self
    }

    pub fn bind(mut self, address: (&str, u16)) -> Self {
        self.address = Some((address.0.to_string(), address.1));
        self
    }

    pub async fn run(self) {
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
                                    if let Some(route) = router_arc.get((method, path)) {
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

    async fn handle_stream(mut stream: tokio::net::TcpStream) -> Result<(Method, String,  tokio::net::TcpStream), Box<dyn Error>> {
        let mut buf_reader = BufReader::new(&mut stream);
        // let mut http_request = DisplayableVec(Vec::new());

        let mut first_line = String::new();
        buf_reader.read_line(&mut first_line).await?;

        let tokens: Vec<&str> = first_line.split_whitespace().collect();
        if tokens.len() != 3 {
            return Err("Invalid HTTP request line".into());
        }

        let method = match tokens[0] {
            "GET" => Method::GET,
            _ => return Err("Unsupported HTTP method".into()),
        };

        let path = tokens[1].to_string();
        log!(Level::Info, "Request Method: {}, Path: {}", method, path);

        Ok((method, path, stream))
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

