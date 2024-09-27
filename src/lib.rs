pub mod http;
pub mod router;
pub mod state;
mod displayeble;

use std::any::Any;
use std::collections::HashMap;
use std::error::Error;
use std::sync::Arc;
use tokio::io::{AsyncBufReadExt, AsyncReadExt, BufReader};
use tracing_log::log::{log, Level};
use crate::displayeble::DisplayableVec;
use crate::http::method::Method;
use crate::http::request::{Request, RequestBuilder};
use crate::router::route::Route;
use crate::router::Router;
use crate::http::status::StatusCode;
use crate::http::response::ResponseBuilder;

pub struct NuttServer{
    address: Option<(String, u16)>,
    router: Router,
    states: HashMap<String, Box<dyn Any + Send + Sync>>
}

impl NuttServer{
    pub fn new() -> Self {
        Self {
            address: None,
            router: Router::new(),
            states: HashMap::new()
        }
    }

    pub fn routes(mut self, routes: Vec<Route>) -> Self {
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
                        tokio::task::spawn(async move {
                            match Self::handle_stream(stream).await {
                                Ok((method, path, stream, req)) => {
                                    if let Some(route) = router_arc.get((method, path)) {
                                        route.run_fabric(stream, req)
                                    } else {
                                        stream.try_write(not_found!().to_string().as_bytes()).unwrap();
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

    async fn handle_stream(mut stream: tokio::net::TcpStream) -> Result<(Method, String,  tokio::net::TcpStream, Request), Box<dyn Error>> {
        let mut buf_reader = BufReader::new(&mut stream);

        let mut bytes_buff = [0;4096];
        buf_reader.read(&mut bytes_buff).await?;

        let request = String::from_utf8_lossy(&bytes_buff).to_string();

        let tokens: Vec<&str> = request.lines().nth(0).unwrap().split_whitespace().collect();
        if tokens.len() != 3 {
            return Err("Invalid HTTP request line".into());
        }

        let method = match tokens[0] {
            "GET" => Method::GET,
            "POST" => Method::POST,
            "PUT" => Method::PUT,
            "DELETE" => Method::DELETE,
            _ => return Err("Unsupported HTTP method".into()),
        };

        let path = tokens[1].to_string();
        let request = request.replace("\0", "");

        let mut headers = DisplayableVec(vec![]);
        let mut i = 1;
        let mut is_header = true;
        let mut body = String::new();
        loop {
            if let Some(line) = request.lines().nth(i) {
                if line == "" {is_header = false}
                if is_header { headers.0.push(line.to_string()); }
                else {body.push_str(line.trim())}
            }else { break }
            i+=1;
        }
        log!(Level::Info, "Request Method: {}, Path: {}, Headers: {}, Body: {}", method, path, headers, body);

        Ok((method.clone(), path, stream, RequestBuilder::new(method, serde_json::to_value(body).unwrap()).build()))
    }

}