use tokio::net::TcpStream;
use tokio_rustls::server::TlsStream;

pub mod stream_reader;
pub mod stream_handler;
pub mod tls_stream;

pub trait Stream {}
impl Stream for TlsStream<TcpStream> {}
impl Stream for TcpStream {}