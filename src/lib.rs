pub mod http;
pub mod modules;

use crate::http::cookie::{CookieJar, CookieReq};
use crate::http::method::Method;
use crate::http::request::{Request, RequestBuilder};
use crate::http::response::ResponseBuilder;
use crate::http::status::StatusCode;
use crate::modules::displayable::DisplayableVec;
use crate::modules::router::route::Route;
use crate::modules::router::Router;
use crate::modules::session::cookie_session::CookieSession;
use crate::modules::session::{Session, SessionType};
use crate::modules::state::State;
use crate::modules::stream_reader::StreamReader;
use serde::Deserialize;
use std::any::Any;
use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::path::{Path, PathBuf};
use std::sync::{Arc, RwLock};
use rustls::pki_types::{CertificateDer, PrivateKeyDer};
use rustls::pki_types::pem::PemObject;
use tokio::io::{AsyncBufReadExt, AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use tokio_rustls::rustls::ServerConfig;
use tokio_rustls::{TlsAcceptor, server::TlsStream};
use tracing_log::log::{log, Level};
use anyhow::Result;

pub trait Stream {}

impl Stream for TlsStream<TcpStream> {}
impl Stream for TcpStream {}

pub struct NuttServer {
    address_dev: Option<(String, u16)>,
    address_release: Option<(String, u16)>,
    router: Router,
    states: Arc<RwLock<HashMap<String, Box<dyn Any + Send + Sync>>>>,
    session: Option<Session>,
    tls_certs: Option<(String, String)>
}

impl Default for NuttServer {
    fn default() -> Self {
        Self::new()
    }
}

impl NuttServer {
    pub fn new() -> Self {
        Self {
            address_dev: None,
            address_release: None,
            router: Router::new(),
            states: Arc::new(RwLock::new(HashMap::new())),
            session: None,
            tls_certs: None,
        }
    }

    pub fn routes(mut self, routes: Vec<Route>) -> Self {
        for route in routes {
            self.router.insert(route.get(), route)
        }
        self
    }

    pub fn bind_dev(mut self, address: (&str, u16)) -> Self {
        self.address_dev = Some((address.0.to_string(), address.1));
        self
    }

    pub fn bind_release(mut self, address: (&str, u16)) -> Self {
        self.address_release = Some((address.0.to_string(), address.1));
        self
    }

    pub fn state<T: Sync + Send + 'static + for<'a> Deserialize<'a>>(
        self,
        state: (String, State<T>),
    ) -> Self {
        self.states
            .try_write()
            .unwrap()
            .insert(state.0, Box::new(state.1));
        self
    }

    pub fn session(mut self, session_type: SessionType) -> Self {
        match session_type {
            SessionType::Cookie => self.session = Some(Session::Cookie(CookieSession::new())),
        }
        self
    }

    pub fn set_tls_certs(mut self, certs: Option<(&str, &str)>) -> Self {
        if let Some(certs) = certs {
            self.tls_certs = Some((certs.0.to_string(), certs.1.to_string()))
        }
        self
    }

    pub async fn run(self) -> Result<()> {
        tracing_subscriber::fmt::init();
        let address = if cfg!(not(debug_assertions)) && self.address_release.is_some() {
            self.address_release
        } else {
            self.address_dev
        };
        if let Some(address) = address {
            let (cert, key) = self.tls_certs.unwrap();
            let certs = Self::load_certs(&PathBuf::from(cert.clone()));
            let key = Self::load_private_key(&PathBuf::from(key.clone()));

            // Configure the server with the certificate and private key
            let mut config = ServerConfig::builder()
                .with_no_client_auth() // No client certificate authentication
                .with_single_cert(certs, key).unwrap();

            let acceptor = TlsAcceptor::from(Arc::new(config));

            let listener = tokio::net::TcpListener::bind(format!("{}:{}", address.0, address.1))
                .await
                .unwrap();
            log!(Level::Info, "Server started on {}:{}", address.0, address.1);
            let router = Arc::new(self.router);
            let states = self.states.clone();
            let session = Arc::new(self.session);
            loop {
                let router_arc = router.clone();
                let states_arc = states.clone();
                let session_arc = session.clone();
                match listener.accept().await {
                    Ok((stream, _)) => {
                        let acceptor_ = acceptor.clone();
                        tokio::task::spawn(async move {
                            if let Ok(stream) = acceptor_.accept(stream).await {
                                match Self::handle_stream(stream).await {
                                    Ok((method, path, mut stream, mut req)) => {
                                        if let Some(route) = router_arc.get((method, path)) {
                                            req.set_states(states_arc.clone());
                                            req.set_session(session_arc.clone());
                                            route.run_fabric(stream, req)
                                        } else {
                                            stream
                                                .write(not_found!().to_string().as_bytes()).await?;
                                        }
                                    }
                                    Err(e) => log!(Level::Error, "Error handling stream: {}", e),
                                }
                            }
                            anyhow::Ok(())
                        });
                    }
                    Err(e) => {
                        log!(Level::Error, "Failed to accept connection: {}", e);
                    }
                }
            }
        } else {
            panic!("Server don't have address")
        }
    }

    async fn handle_stream<T: Stream + AsyncReadExt + Unpin>(
        mut stream: T,
    ) -> Result<(Method, String, T, Request)> {
        let request = StreamReader::new(&mut stream).read_req().await;
        let tokens: Vec<&str> = request.lines().nth(0).unwrap().split_whitespace().collect();
        if tokens.len() != 3 {
            return Err(anyhow::Error::msg("Invalid HTTP request line"));
        }

        let method = match tokens[0] {
            "GET" => Method::GET,
            "POST" => Method::POST,
            "PUT" => Method::PUT,
            "DELETE" => Method::DELETE,
            _ => return Err(anyhow::Error::msg("Unsupported HTTP method")),
        };

        let path = tokens[1].to_string();

        let mut headers = DisplayableVec(vec![]);
        let mut i = 1;
        let mut is_header = true;
        let mut body = String::new();
        while let Some(line) = request.lines().nth(i) {
            if line.is_empty() {
                is_header = false
            }
            if is_header {
                headers.0.push(line.to_string());
            } else {
                body.push_str(line.trim())
            }
            i += 1;
        }

        let mut cookies = CookieJar::new();

        for header in &headers.0 {
            if header.starts_with("Cookie: ") {
                for cookie in header[8..].split(";") {
                    let eq_pos = cookie.find("=").unwrap();
                    cookies.push_cookie(
                        &cookie[..eq_pos],
                        CookieReq::new(cookie[eq_pos + 1..].to_string()),
                    )
                }
            }
        }

        log!(
            Level::Info,
            "Request Method: {}, Path: {}, Headers: {}, Body: {}",
            method,
            path,
            headers,
            body
        );

        Ok((
            method.clone(),
            path,
            stream,
            RequestBuilder::new(method, serde_json::to_value(body).unwrap())
                .set_cookie_jar(cookies)
                .build(),
        ))
    }

    fn load_certs(filename: &Path) -> Vec<CertificateDer<'static>> {
        CertificateDer::pem_file_iter(filename)
            .expect("cannot open certificate file")
            .map(|result| result.unwrap())
            .collect()
    }

    fn load_private_key(filename: &Path) -> PrivateKeyDer<'static> {
        PrivateKeyDer::from_pem_file(filename).expect("cannot read private key file")
    }
}
