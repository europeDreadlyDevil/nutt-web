use std::path::{Path, PathBuf};
use std::sync::Arc;
use rustls::pki_types::{CertificateDer, PrivateKeyDer};
use rustls::pki_types::pem::PemObject;
use tokio_rustls::{TlsAcceptor};
use anyhow::Result;

pub use rustls::ServerConfig;

pub struct TlsConfigWrapper {
    server_config: ServerConfig,
    tls_acceptor: TlsAcceptor
}

impl TlsConfigWrapper {
    pub fn new_with_self_config(server_config: ServerConfig) -> Self {
        Self {
            tls_acceptor: TlsAcceptor::from(Arc::new(server_config.clone())),
            server_config,
        }
    }

    pub fn clone_acceptor(&self) -> TlsAcceptor {
        self.tls_acceptor.clone()
    }

    pub fn new_with_certs_and_key<P: AsRef<Path>, U: AsRef<Path>>(certs_path: P, key_path: U) -> Result<Self> {
        let certs = Self::load_certs(&PathBuf::from(certs_path.as_ref()))?;
        let key = Self::load_private_key(&PathBuf::from(key_path.as_ref()))?;

        let config = ServerConfig::builder()
            .with_no_client_auth()
            .with_single_cert(certs, key)?;

        Ok(Self::new_with_self_config(config))
    }
    pub fn load_certs(filename: &Path) -> Result<Vec<CertificateDer<'static>>> {
        let mut certs = vec![];
        for cert in CertificateDer::pem_file_iter(filename)? {
            certs.push(cert?);
        }
        Ok(certs)
    }

    pub fn load_private_key(filename: &Path) -> Result<PrivateKeyDer<'static>> {
        Ok(PrivateKeyDer::from_pem_file(filename)?)
    }
}