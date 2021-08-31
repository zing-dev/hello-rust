use actix_web::{Error, App, HttpRequest, HttpResponse, HttpServer, Responder, web, get};
use std::future::{Ready, ready};
use futures::{future::ok, stream::once};
use serde::Serialize;

async fn index1(_req: HttpRequest) -> &'static str {
    "Hello world!"
}

async fn index2(_req: HttpRequest) -> String {
    "Hello world!".to_owned()
}

// You can also change the signature to return impl Responder which works well if more complex types are involved.
async fn index3(_req: HttpRequest) -> impl Responder {
    web::Bytes::from_static(b"Hello world!")
}

#[derive(Serialize)]
struct User {
    id: u8,
    name: &'static str,
}

// Responder
impl Responder for User {
    type Error = Error;
    type Future = Ready<Result<HttpResponse, Error>>;

    fn respond_to(self, _req: &HttpRequest) -> Self::Future {
        let body = serde_json::to_string(&self).unwrap();

        // Create response and set content type
        ready(Ok(HttpResponse::Ok()
            .content_type("application/json")
            .body(body)))
    }
}

async fn index4() -> impl Responder {
    User { id: 1, name: "user" }
}

#[get("/stream")]
async fn stream() -> HttpResponse {
    let body = once(ok::<_, Error>(web::Bytes::from_static(b"test")));

    HttpResponse::Ok()
        .content_type("application/json")
        .streaming(body)
}

#[actix_web::main]
#[test]
async fn handler() -> std::io::Result<()> {
    HttpServer::new(|| App::new()
        .route("/1", web::get().to(index1))
        .route("/2", web::get().to(index2))
        .route("/3", web::get().to(index3))
        .route("/4", web::get().to(index4))
        .service(stream)
    )
        .bind("0.0.0.0:9090")?
        .run()
        .await
}