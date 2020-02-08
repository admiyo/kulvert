
use std::fs::File;
use std::io::BufReader;
use http::HeaderValue;

use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer, Result};
 use rustls::internal::pemfile::{certs, rsa_private_keys};
use rustls::{RootCertStore, AllowAnyAnonymousOrAuthenticatedClient, ServerConfig};

mod versions;
mod html;

async fn versions(req: HttpRequest) -> Result<HttpResponse> {        
    println!("REQ: {:?}", req);
    let v = versions::get_versions();
    let j = serde_json::to_string(&v).unwrap();
    Ok(HttpResponse::Ok()
        .content_type("application/json; charset=utf-8")
        .body(j))
}

async fn v3(req: HttpRequest) -> Result<HttpResponse> {    
    let accepts = req.headers().get("accept");
    println!("REQ accepts: {:?}", accepts);

    let v = versions::get_v3();
    let j = serde_json::to_string(&v).unwrap();
    Ok(HttpResponse::Ok()
        .content_type("application/json; charset=utf-8")
        .body(j))

}

fn select_render(value: &HeaderValue) -> String{
    let split = value.to_str().unwrap().split(",");
    for s in split {
        if s == "text/html"{
            return html::content();
        }else if s == "application/xhtml+xml" {
            return html::content();
        }else if s==  "application/json" {
            let v = versions::get_versions();
            let j = serde_json::to_string(&v).unwrap();
            return j;
        }
    }
    //
    return "No Known Content Type Accepted".to_string();
}

async fn index(req: HttpRequest) -> Result<HttpResponse> {

    let r = req.headers().get("accept");

    let s: String;
    match r {
        Some(accepts) => s = select_render(accepts),
        None =>  s = "No accept header".to_string(),
    }    
    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(s))
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
            .service(web::resource("/index").to(index))
             })
        .bind_rustls("127.0.0.1:8443", config)?
        .run()
        .await
}
