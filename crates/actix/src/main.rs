use serde::{Deserialize, Serialize};

pub mod start {
    use actix_web::{App, get, HttpResponse, HttpServer, post, Responder, web};

    #[get("/")]
    async fn hello() -> impl Responder {
        HttpResponse::Ok().body("Hello world!")
    }

    #[post("/echo")]
    async fn echo(req_body: String) -> impl Responder {
        HttpResponse::Ok().body(req_body)
    }

    async fn to_hello() -> impl Responder {
        HttpResponse::Ok().body("Hey there!")
    }

    #[actix_web::main]
    #[test]
    async fn start() -> std::io::Result<()> {
        // 实例化HTTP服务
        HttpServer::new(|| {
            App::new()
                .service(hello)
                .service(echo)
                .route("hi", web::get().to(to_hello))
        }).bind("0.0.0.0:9090")?
            .run()
            .await
    }

    mod scope {
        use actix_web::{App, HttpResponse, HttpServer, Responder, web};

        async fn index() -> impl Responder {
            "Hello world!"
        }

        #[actix_web::main]
        #[test]
        async fn scope() -> std::io::Result<()> {
            HttpServer::new(|| {
                // prefixes all resources and routes attached to it...
                App::new().service(web::scope("/app")
                                       // ...so this handles requests for `GET /app/index.html`
                                       .route("/", web::get().to(|| HttpResponse::Forbidden().body("you are forbidden")))
                                       .route("/index.html", web::get().to(index)),
                )
            })
                .bind("0.0.0.0:9090")?
                .run()
                .await
        }
    }

    mod state {
        use std::sync::Mutex;

        use actix_web::{App, get, HttpServer, web, HttpResponse};
        use serde::{Deserialize, Serialize};

        // This struct represents state
        struct AppState {
            app_name: String,
        }

        struct AppStateWithCounter {
            counter: Mutex<i32>, // <- Mutex is necessary to mutate safely across threads
        }

        #[derive(Serialize, Deserialize, Clone)]
        struct User {
            id: u8,
            name: String,
            languages: Vec<String>,
        }

        /// 不可变共享数据
        #[get("/")]
        async fn index(data: web::Data<AppState>) -> String {
            let app_name = &data.app_name; // <- get app_name

            format!("Hello {}!", app_name) // <- response with app_name
        }

        /// 可变共享数据
        #[get("/c")]
        async fn count(data: web::Data<AppStateWithCounter>) -> String {
            let mut counter = data.counter.lock().unwrap(); // <- get counter's MutexGuard
            *counter += 1; // <- access counter inside MutexGuard

            format!("Request number: {}", counter) // <- response with count
        }

        #[get("/item")]
        async fn users() -> String {
            let user = User {
                id: 1,
                name: "zing".to_string(),
                languages: vec!["rust".to_owned(), "go".to_owned(), "java".to_owned()],
            };
            return match serde_json::to_string(&user) {
                Ok(s) => s,
                Err(err) => format!("解析失败:{}", err.to_string())
            };
        }

        #[actix_web::main]
        #[test]
        async fn state() -> std::io::Result<()> {
            HttpServer::new(|| {
                let scope = web::scope("/users").service(users)
                    .service(web::resource("/ok").to(|| HttpResponse::Ok().body("ok")));
                App::new()
                    .data(AppState {
                        app_name: String::from("Actix-web"),
                    }).data(AppStateWithCounter {
                    counter: Mutex::new(1)
                })
                    .service(index)
                    .service(count)
                    .service(scope)
            })
                .bind("0.0.0.0:9090")?
                .run()
                .await
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct Point {
    x: i32,
    y: i32,
}

#[test]
fn json() {
    let point = Point { x: 1, y: 2 };

    let serialized = serde_json::to_string(&point).unwrap();
    println!("serialized = {}", serialized);

    let deserialized: Point = serde_json::from_str(&serialized).unwrap();
    println!("deserialized = {:?}", deserialized);
}


pub mod app {
    use actix_web::{App, HttpServer, web, guard, HttpResponse};
    use actix_cors::Cors;

    // this function could be located in a different module
    fn config(cfg: &mut web::ServiceConfig) {
        cfg.service(
            web::resource("/app")
                .route(web::get().to(|| HttpResponse::Ok().body("app")))
                .route(web::head().to(|| HttpResponse::MethodNotAllowed())),
        );
    }

    // this function could be located in a different module
    fn scoped_config(cfg: &mut web::ServiceConfig) {
        cfg.service(
            web::resource("/test")
                .route(web::get().to(|| HttpResponse::Ok().body("test")))
                .route(web::head().to(|| HttpResponse::MethodNotAllowed())),
        );
    }

    #[actix_web::main]
    #[test]
    async fn run() -> std::io::Result<()> {
        HttpServer::new(|| {
            let cors = Cors::default()
                // .allowed_origin("https://www.rust-lang.org/")
                .allow_any_origin()
                // .allowed_origin_fn(|origin, _req_head| {
                //     origin.as_bytes().ends_with(b".rust-lang.org")
                // })
                // .allowed_methods(vec!["GET", "POST"])
                // .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
                // .allowed_header(http::header::CONTENT_TYPE)
                .allow_any_method()
                .max_age(3600);
            App::new()
                .configure(config)
                .wrap(cors)
                .service(web::scope("/api").configure(scoped_config))
                .service(
                    web::scope("/")
                        .guard(guard::Header("Host", "www.rust-lang.org"))
                        .route("", web::to(|| HttpResponse::Ok().body("www"))),
                )
                .service(
                    web::scope("/")
                        .guard(guard::Header("Host", "localhost:9090"))
                        .route("", web::to(|| HttpResponse::Ok().body("localhost ok..."))),
                )
                .route("/", web::to(|| HttpResponse::Ok().body("default route")))
        })
            .bind("0.0.0.0:9090")?
            .run()
            .await
    }
}

pub mod server;

fn main() {}