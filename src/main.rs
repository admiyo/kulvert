use std::fs::File;
use std::io::BufReader;

use actix_web::{web, App,  HttpRequest, HttpServer, Responder};

use rustls::internal::pemfile::{certs, rsa_private_keys};
use rustls::{RootCertStore, AllowAnyAnonymousOrAuthenticatedClient, ServerConfig};

mod versions;


async fn versions(req: HttpRequest) -> impl Responder {

    
    println!("REQ: {:?}", req);

    let v = versions::get_versions();
    let j = serde_json::to_string(&v);
    return j;
}

async fn v3(req: HttpRequest) -> impl Responder {
    println!("REQ: {:?}", req);

    let v = versions::get_v3();
    let j = serde_json::to_string(&v);
    return j;
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    
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


    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "actix_web=info");
    }
    env_logger::init();

    
    HttpServer::new(|| {
        App::new()
            .service(web::resource("/").to(versions))
            .service(web::resource("/v3").to(v3))
    })
        .bind_rustls("127.0.0.1:8443", config)?
        .run()
        .await
}
