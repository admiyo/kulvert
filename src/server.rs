use http::HeaderValue;
use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer, Result};
use serde::{Serialize};

async fn v3(req: HttpRequest) -> Result<HttpResponse> {
    let v = super::versions::get_v3();
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
                      .body(super::html::to_string(v).unwrap()))
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
    let v = super::versions::get_versions();
    match r {
        Some(accepts) => return select_render(accepts, &v),
        None => return  Ok(HttpResponse::Ok()
                           .content_type("text/html; charset=utf-8")
                           .body("no accepts header"))
    }
}

async fn namespaces(req: HttpRequest) -> Result<HttpResponse> {
    let r = req.headers().get("accept");
    let v = super::access::get_namespace();
    match r {
        Some(accepts) => return select_render(accepts, &v),
        None => return  Ok(HttpResponse::Ok()
                           .content_type("text/html; charset=utf-8")
                           .body("no accepts header"))
    }
}

async fn identity_providers(req: HttpRequest) -> Result<HttpResponse> {
    let r = req.headers().get("accept");
    let v = super::identity::get_providers();
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

    let v = super::identity::get_provider(id);
    match r {
        Some(accepts) => return select_render(accepts, &v),
        None => return  Ok(HttpResponse::Ok()
                           .content_type("text/html; charset=utf-8")
                           .body("no accepts header"))
    }
}


#[actix_rt::main]
pub async fn server_main(tls: bool) -> std::io::Result<()> {
    super::config::set_logging();
    
    let mut server = HttpServer::new(|| {
        App::new()
            .service(web::resource("/").to(versions))
            .service(web::resource("/v3").to(v3))
            .service(web::resource("/v3/").to(v3))
            .service(web::resource("/v3/identity_providers").
                     to(identity_providers))
            .service(web::resource("/v3/identity_providers/{id}").
                     to(identity_provider ))
            .service(web::resource("/v3/namespaces").
                     to(namespaces))
    });

    server = {
        if tls{
            let conf = super::config::load_config();
            server.bind_rustls("0.0.0.0:8443", conf)?
        }else{
            server.bind("0.0.0.0:8080")?
        }
    };
    server.run().await
}
