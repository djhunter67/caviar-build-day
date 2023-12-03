use actix_files::NamedFile;
use actix_web::middleware::Logger;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use log::debug;
use simplelog::{CombinedLogger, Config, File, LevelFilter, WriteLogger};

async fn send_css_file() -> HttpResponse {
    let css_file_data = include_str!("static/css/style.css");
    HttpResponse::Ok()
        .content_type("text/css")
        .body(css_file_data)
}

async fn send_fav_icon() -> actix_web::Result<actix_files::NamedFile> {
    Ok(NamedFile::open("static/assets/images/favicon.png")?)
}

#[actix_web::main]
async fn main() {
    match CombinedLogger::init(vec![WriteLogger::new(
        log::LevelFilter::Debug,
        simplelog::Config::default(),
        File::create("caviar.log").unwrap(),
    )]) {
        Ok(_) => debug!("Logger initialized"),
        Err(e) => debug!("Logger failed to initialize: {}", e),
    }

    let _ = HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .service(fs::Files::new("/static", ".").show_files_listing())
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
