use actix_web::{get, web, App, HttpServer, Responder};
use serde::Serialize;

#[get("/")]
async fn index() -> impl Responder {
    format!("Hello actix")
}

#[get("/{id}/{name}/index.html")]
async fn hello(info: web::Path<(u32, String)>) -> impl Responder {
    format!("Hello {}! id:{}", info.1, info.0)
}

#[derive(Serialize)]
struct MyObj {
    name: String,
    age: u8,
}

#[get("/json")]
async fn json() -> impl Responder {
    web::Json(MyObj {
        name: "Name".to_string(),
        age: 26,
    })
    .with_header("x-version", "1.2.3")
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(hello).service(index).service(json))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
