#[macro_use]
extern crate diesel;
mod model;
mod controller;
mod utility;

use actix_web::{web, App, HttpServer};
use actix_identity::{CookieIdentityPolicy, IdentityService};
use actix_web::middleware::Logger;
use diesel::pg::PgConnection;
use diesel::{r2d2::ConnectionManager};
use dotenv::dotenv;
use std::env;
use tera::Tera;


include!(concat!(env!("OUT_DIR"), "/generated.rs"));

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    // Setup DB
    let db_wait_time: u64 = env::var("DB_WAIT_TIME").expect("DB_WAIT_TIME is required.").parse().expect("Couldn't parse db_wait_time.");
    let db_url = env::var("DB_URL").expect("DB_URL is required.");
    let connection_manager = ConnectionManager::<PgConnection>::new(db_url);

    // Wait for DB to initialize
    std::thread::sleep(std::time::Duration::new(db_wait_time, 0));
    let pool = r2d2::Pool::builder().build(connection_manager).expect("Failed to create DB pool.");

    // Get binding
    let host = std::env::var("WEB_HOST").expect("WEB_HOST is required.");
    let port = std::env::var("WEB_PORT").expect("WEB_PORT is required.");

    // Initiate logger
    env_logger::init();

    // Configure server
    HttpServer::new(move || {

        // Register bundles
        let generated = generate();

        // Register templates
        let mut tera = match Tera::new("src/view/**/*") {
            Ok(t) => t,
            Err(e) => {
                println!("Tera parsing error(s): {}", e);
                ::std::process::exit(1);
            }
        };
        tera.full_reload().expect("Error running auto reload with Tera");

        // Register routes
        App::new()
            .wrap(Logger::default())
            .wrap(IdentityService::new(
                CookieIdentityPolicy::new(&[0;32])
                .name("auth-cookie")
                .secure(false)
            ))
            // Templates
            .data(tera)
            .data(pool.clone())
            .service(actix_web_static_files::ResourceFiles::new("/bundle", generated,))       
            // Admin 
            .route("/navbar.html", web::get().to(controller::home::navbar))
            .route("/signup.html", web::get().to(controller::account::signup))
            .route("/login.html", web::get().to(controller::account::login))
            .route("/register_user", web::post().to(controller::account::register_user))       
            .route("/authenticate", web::post().to(controller::account::authenticate))
            .route("/create_post", web::post().to(controller::admin::create_post))
            .route("/delete_post", web::post().to(controller::admin::delete_post))
            .route("/edit_post", web::post().to(controller::admin::edit_post))
            .route("/edit_user", web::post().to(controller::admin::edit_user))
            .route("/post_comment", web::post().to(controller::posts::post_comment))
            .route("/logout", web::to(controller::account::logout))
            .route("/editor", web::to(controller::admin::editor))
            .route("/users", web::to(controller::admin::users))
            // Home
            .route("/", web::get().to(controller::home::home))
            .route("/contact", web::get().to(controller::home::contact))
            .route("/home", web::get().to(controller::home::home))
            // Posts
            .route("/blog", web::get().to(controller::posts::blog))
            .route("/git", web::get().to(controller::posts::git))
            // Errors
            .route("/oof", web::get().to(controller::error::oof))
            .route("/forbidden", web::get().to(controller::error::forbidden))
            .route("/unavailable", web::get().to(controller::error::unavailable))
            .default_service(web::get().to(controller::error::not_found))
    })
    .bind(format!("{}:{}", host, port))?
    .run()
    .await
} 