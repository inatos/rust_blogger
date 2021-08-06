use actix_web::{web, HttpResponse, Responder};
use tera::{Tera, Context};


pub async fn forbidden(tera: web::Data<Tera>) -> HttpResponse {
    let mut data = Context::new();
    data.insert("title", "Forbidden");
    let rendered = tera.render("error/forbidden.html", &data).unwrap();
    HttpResponse::Ok().body(rendered)
}

pub async fn not_found(tera: web::Data<Tera>) -> HttpResponse {
    let mut data = Context::new();
    data.insert("title", "Not Found");
    let rendered = tera.render("error/not_found.html", &data).unwrap();
    HttpResponse::Ok().body(rendered)
}

pub async fn oof(error_message: String, tera: web::Data<Tera>) -> HttpResponse {
    let mut data = Context::new();
    data.insert("title", "Oof");
    data.insert("error_message", &error_message);
    let rendered = tera.render("error/oof.html", &data).unwrap();
    HttpResponse::Ok().body(rendered)
}

pub async fn unavailable(tera: web::Data<Tera>) -> impl Responder {
    let mut data = Context::new();
    data.insert("title", "Unavailable");
    let rendered = tera.render("error/unavailable.html", &data).unwrap();
    HttpResponse::Ok().body(rendered)
}