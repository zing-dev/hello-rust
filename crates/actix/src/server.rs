use std::sync::mpsc;
use std::thread;
use std::thread::sleep;
use std::time::Duration;

use actix_web::{App, HttpResponse, HttpServer, rt::System, web};

#[actix_web::main]
#[test]
async fn main() {
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