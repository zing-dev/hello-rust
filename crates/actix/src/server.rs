use std::sync::mpsc;
use std::thread;
use std::thread::sleep;
use std::time::Duration;

use actix_web::{App, get, HttpRequest, HttpResponse, HttpServer, Responder, rt::System, web};
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};

#[actix_web::main]
#[test]
async fn server() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let sys = System::new("http-server");

        let srv = HttpServer::new(|| {
            App::new().route("/", web::get().to(|| HttpResponse::Ok()))
        })
            .bind("0.0.0.0:9090")?
            .shutdown_timeout(60) // <- Set shutdown timeout to 60 seconds
            .run();
        thread::spawn(|| {
            let mut i = 0;
            loop {
                if i >= 30 { break; }
                sleep(Duration::from_secs(1));
                println!("hello");
                i += 1;
            }
        });
        sleep(Duration::from_secs(10));
        let _ = tx.send(srv);
        println!("send...");
        sys.run()
    });

    let srv = rx.recv().unwrap();
    println!("receive...");
    // pause accepting new connections
    srv.pause().await;
    println!("pause...");

    // resume accepting new connections
    srv.resume().await;
    println!("resume...");

    // stop server
    srv.stop(true).await;
    println!("stop...");
}


#[actix_web::main]
#[test]
async fn ssl() -> std::io::Result<()> {
    #[get("/")]
    async fn index(_: HttpRequest) -> impl Responder {
        "welcome ssl"
    }
    // `openssl req -x509 -newkey rsa:4096 -nodes -keyout key.pem -out cert.pem -days 365
    let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
    builder.set_private_key_file("key.pem", SslFiletype::PEM).unwrap();
    builder.set_certificate_chain_file("cert.pem").unwrap();
    HttpServer::new(|| App::new().service(index)).keep_alive(Some(20)).bind_openssl("0.0.0.0:9090", builder)?.run().await
}