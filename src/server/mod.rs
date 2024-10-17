pub mod stream;
mod logger;

use crate::http::cookie::{CookieJar, CookieReq};
use crate::http::method::Method;
use crate::http::request::{RequestBuilder};
use crate::http::response::ResponseBuilder;
use crate::http::status::StatusCode;
use crate::external::displayable::DisplayableVec;
use crate::modules::router::route::Route;
use crate::modules::router::Router;
use crate::modules::session::cookie_session::CookieSession;
use crate::modules::session::{Session, SessionType};
use crate::modules::state::State;
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
use crate::not_found;
use crate::server::stream::stream_handler::StreamHandler;
use crate::server::stream::stream_reader::StreamReader;

#[derive(Default)]
pub struct NuttServer {
    address_dev: Option<(String, u16)>,
    address_release: Option<(String, u16)>,
    router: Router,
    states: Arc<RwLock<HashMap<String, Box<dyn Any + Send + Sync>>>>,
    session: Option<Session>,
    tls_certs: Option<(String, String)>
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
                .with_single_cert(certs, key)?;

            let acceptor = TlsAcceptor::from(Arc::new(config));

            let listener = tokio::net::TcpListener::bind(format!("{}:{}", address.0, address.1))
                .await?;
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
                                match StreamHandler::handle_stream(stream).await {
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

