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
                App::new().service(
                    // prefixes all resources and routes attached to it...
                    web::scope("/app")
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

        use actix_web::{App, get, HttpServer, web};
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
                let scope = web::scope("/users").service(users);
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

fn main() {
    let point = Point { x: 1, y: 2 };

    let serialized = serde_json::to_string(&point).unwrap();
    println!("serialized = {}", serialized);

    let deserialized: Point = serde_json::from_str(&serialized).unwrap();
    println!("deserialized = {:?}", deserialized);
}