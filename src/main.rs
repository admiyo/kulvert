use actix_web::{get, web, App,  HttpRequest, HttpServer, Responder};

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


#[get("/{id}/{name}/index.html")]
async fn index(info: web::Path<(u32, String)>) -> impl Responder {
    format!("Hello {}! id:{}", info.1, info.0)
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(index)
            .service(web::resource("/").to(versions))
            .service(web::resource("/v3").to(v3))
    })
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
