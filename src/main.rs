use std::fs::File;
use std::io::BufReader;
use http::HeaderValue;



use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer, Result};
use rustls::internal::pemfile::{certs, rsa_private_keys};
use rustls::{RootCertStore, AllowAnyAnonymousOrAuthenticatedClient, ServerConfig};

use serde::{Serialize};

mod access;
mod html;
mod identity;
mod versions;
mod links;

async fn v3(req: HttpRequest) -> Result<HttpResponse> {
    let v = versions::get_v3();
    let r = req.headers().get("accept");
    match r {
        Some(accepts) => return select_render(accepts, &v),
        None => return  Ok(HttpResponse::Ok()
                           .content_type("text/html; charset=utf-8")
                           .body("no accepts header"))
    }
}

fn select_render<T>(value: &HeaderValue, v: &T) -> Result<HttpResponse>
where
    T: Serialize,
{
    let split = value.to_str().unwrap().split(",");
    for s in split {
        if s == "text/html"{
            return Ok(HttpResponse::Ok()
                      .content_type("text/html; charset=utf-8")
                      .body(html::to_string(v).unwrap()))
        }else if s==  "application/json" {
            let j = serde_json::to_string(v).unwrap();
            return Ok(HttpResponse::Ok()
               .content_type("application/json; charset=utf-8")
               .body(j));
        }
    }
    Ok(HttpResponse::Ok()
       .content_type("text/html; charset=utf-8")
       .body("No Known Content Type Accepted".to_string()))
}

async fn versions(req: HttpRequest) -> Result<HttpResponse> {
    let r = req.headers().get("accept");
    let v = versions::get_versions();
    match r {
        Some(accepts) => return select_render(accepts, &v),
        None => return  Ok(HttpResponse::Ok()
                           .content_type("text/html; charset=utf-8")
                           .body("no accepts header"))
    }
}

async fn namespace(req: HttpRequest) -> Result<HttpResponse> {
    let r = req.headers().get("accept");
    let v = access::get_namespace();
    match r {
        Some(accepts) => return select_render(accepts, &v),
        None => return  Ok(HttpResponse::Ok()
                           .content_type("text/html; charset=utf-8")
                           .body("no accepts header"))
    }
}

async fn identity_providers(req: HttpRequest) -> Result<HttpResponse> {
    let r = req.headers().get("accept");
    let v = identity::get_providers();
    match r {
        Some(accepts) => return select_render(accepts, &v),
        None => return  Ok(HttpResponse::Ok()
                           .content_type("text/html; charset=utf-8")
                           .body("no accepts header"))
    }
}

async fn identity_provider(req: HttpRequest) -> Result<HttpResponse> {
    let r = req.headers().get("accept");

    let id = req.match_info().get("id").unwrap_or("none");
    
    let v = identity::get_provider(id);
    match r {
        Some(accepts) => return select_render(accepts, &v),
        None => return  Ok(HttpResponse::Ok()
                           .content_type("text/html; charset=utf-8")
                           .body("no accepts header"))
    }
}


#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    
    let insecure: bool = true;

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


    let mut server = HttpServer::new(|| {
        App::new()
            .service(web::resource("/").to(versions))
            .service(web::resource("/v3").to(v3))
            .service(web::resource("/v3/").to(v3))
            .service(web::resource("/v3/identity_providers").
                     to(identity_providers))
            .service(web::resource("/v3/identity_providers/{id}").
                     to(identity_provider ))
            
            .service(web::resource("/v3/namespace").
                     to(namespace))
    });

    server = { 
        if insecure{
            server.bind("0.0.0.0:8080")?
        }else{
            server.bind_rustls("0.0.0.0:8443", config)?
        }
    };
    server.run().await
}
