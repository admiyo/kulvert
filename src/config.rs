use std::fs::File;
use std::io::BufReader;
use rustls::{RootCertStore, AllowAnyAnonymousOrAuthenticatedClient, ServerConfig};
use rustls::internal::pemfile::{certs, rsa_private_keys};


pub fn load_config() -> ServerConfig {
           // load ssl keys
    let cert_file = &mut BufReader::new(File::open("cert.pem").unwrap());
    let key_file = &mut BufReader::new(File::open("key.pem").unwrap());
    let cert_chain = certs(cert_file).unwrap();
    let mut keys = rsa_private_keys(key_file).unwrap();
    let mut root_store = RootCertStore::empty();
    root_store.add(&cert_chain[0]).unwrap();

    let mut config = ServerConfig::new(
        AllowAnyAnonymousOrAuthenticatedClient::new(root_store));
    config.set_single_cert(cert_chain, keys.remove(0)).unwrap();

    config
}

pub fn set_logging(){
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "actix_web=info");
    }
    env_logger::init();
}
