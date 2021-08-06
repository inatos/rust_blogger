use actix_web::{web, HttpResponse, Responder};
use actix_identity::Identity;
use diesel::pg::PgConnection;
use diesel::{r2d2::ConnectionManager};
use super::super::utility::DbContext;
use tera::{Tera, Context};
type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;


/**
 * Render contact page.
 */
pub async fn contact(tera: web::Data<Tera>) -> impl Responder {
    let mut data = Context::new();
    data.insert("title", "Contact");

    let rendered = tera.render("contact.html", &data).unwrap();
    HttpResponse::Ok().body(rendered)
}

/**
 * Render home page.
 */
pub async fn home(tera: web::Data<Tera>, pool: web::Data<Pool>) -> impl Responder {
    let mut data = Context::new();
    data.insert("title", "Home");

    let db_context = DbContext::new(&pool);
    match db_context.get_post_latest() {
        Some(p) => {
            data.insert("post", &p);
            data.insert("post_link", &DbContext::get_post_redirect(p.post_category_id));
        },
        _ => ()
    }

    let rendered = tera.render("home.html", &data).unwrap();
    HttpResponse::Ok().body(rendered)
} 

/**
 * Render navbar.
 */
pub async fn navbar(tera: web::Data<Tera>, session_id: Identity) -> impl Responder {
    let mut admin = false;
    let authenticated = match session_id.identity() {
        Some(id) => {
            if id == "admin" { admin = true }
            true
        },
        _ => false
    };

    let mut data = Context::new();
    data.insert("authenticated", &authenticated);
    data.insert("admin", &admin);
    let rendered = tera.render("partial/navbar.html", &data).unwrap();
    return HttpResponse::Ok().body(rendered);
}