use actix_web::{web, App, HttpResponse, HttpServer, Responder};

#[actix_web::main]
async fn main() {
    let _ = HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(index))
            .route("/hello", web::get().to(hello))
    })
    .bind("0.0.0.0:8080")
    .expect("Can not bind to port 8080")
    .run()
    .await;
}

struct Login {
    username: String,
    password: String,
}

struct CreateUser {
    username: String,
    password: String,
    email: String,
}

struct UpdateUser {
    username: String,
    password: String,
    email: String,
}

struct DeleteUser {
    username: String,
}

async fn login() -> impl Responder {
    HttpResponse::Ok().body("Login")
}

async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}
