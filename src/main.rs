use actix_files::NamedFile;
use actix_web::middleware::Logger;
use actix_web::{post, web, App, HttpResponse, HttpServer, Responder};
use askama::Template;
use log::debug;
use serde::Deserialize;
use simplelog::{CombinedLogger, Config, LevelFilter, TermLogger, WriteLogger};
use std::fs::File;
use uuid::Uuid;

async fn send_css_file() -> HttpResponse {
    let css_file_data = include_str!("../static/css/style.css");
    HttpResponse::Ok()
        .content_type("text/css")
        .body(css_file_data)
}

async fn send_fav_icon() -> actix_web::Result<actix_files::NamedFile> {
    Ok(NamedFile::open("../static/assets/images/favicon.png")?)
}

#[actix_web::main]
async fn main() {
    match CombinedLogger::init(vec![
        WriteLogger::new(
            LevelFilter::Debug,
            Config::default(),
            File::create("caviar.log").unwrap(),
        ),
        TermLogger::new(
            LevelFilter::Debug,
            Config::default(),
            simplelog::TerminalMode::Mixed,
            simplelog::ColorChoice::Auto,
        ),
    ]) {
        Ok(_) => debug!("Logger initialized"),
        Err(e) => debug!("Logger failed to initialize: {}", e),
    }

    let _ = HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .route("/favicon", web::get().to(send_fav_icon))
            .route("/styles.css", web::get().to(send_css_file))
            .service(web::resource("/home").to(|| async {
                IndexTemplate {
                    title: "Home",
                    name: "Hunter, Hunter",
                }
            }))
            .service(web::resource("/").to(|| async { LoginTemplate { title: "Login" } }))
    })
    .bind("0.0.0.0:8080")
    .expect("Can not bind to port 8080")
    .run()
    .await;
}

#[derive(Template)]
#[template(path = "login.html")]
struct LoginTemplate {
    title: &'static str,
}

struct CreateUser {
    username: String,
    password: String,
    email: String,
}

#[derive(Debug, Deserialize)]
struct LoginUser {
    username: String,
    password: String,
}

struct UpdateUser {
    username: String,
    password: String,
    email: String,
}

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate<'a> {
    title: &'a str,
    name: &'a str,
}

struct DeleteUser {
    username: String,
}

/// Take in the user login form
#[post("/login")]
async fn login(user_login: web::Json<LoginUser>) -> impl Responder {
    // Assign a session token to the user
    let session_token = Uuid::new_v4();
    let user_login = user_login.into_inner();
    debug!("User login: {:?}", user_login);

    // Return the session token
    HttpResponse::Ok().body(format!("Session token: {}", session_token))
}
