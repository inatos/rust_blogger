use actix_web::{web, HttpResponse, Responder};
use actix_web::http::StatusCode;
use actix_identity::Identity;
use diesel::pg::PgConnection;
use diesel::{r2d2::ConnectionManager};
use super::super::model::{Category, CommentForm, NewComment};
use super::super::utility::DbContext;
use tera::{Tera, Context};
type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;



/**
 * Render blog page.
 */
pub async fn blog(tera: web::Data<Tera>, session_id: Identity, pool: web::Data<Pool>) -> impl Responder {
    let mut data = Context::new();
    data.insert("title", "Blog");
    
    // Check auth
    let authenticated = match session_id.identity() {
        Some(_) => true,
        None => false
    };
    data.insert("authenticated", &authenticated);

    // Get posts and their comments
    let db_context = DbContext::new(&pool);
    let posts_info = db_context.get_posts_info(Category::Blog as i32);
    data.insert("posts_info", &posts_info);

    let rendered = tera.render("post_template.html", &data).unwrap();
    HttpResponse::Ok().body(rendered)
}


/**
 * Render git page.
 */
pub async fn git(tera: web::Data<Tera>, session_id: Identity, pool: web::Data<Pool>) -> impl Responder {
    let mut data = Context::new();
    data.insert("title", "Git");
    
    // Check auth
    let authenticated = match session_id.identity() {
        Some(_) => true,
        None => false
    };
    data.insert("authenticated", &authenticated);

    // Get posts and their comments
    let db_context = DbContext::new(&pool);
    let posts = db_context.get_posts_by_category(Category::Git as i32);
    data.insert("posts", &posts);

    let rendered = tera.render("git.html", &data).unwrap();
    HttpResponse::Ok().body(rendered)
}


/**
 * Create comment for a post.
 */
pub async fn post_comment(comment_form: web::Form<CommentForm>, session_id: Identity, pool: web::Data<Pool>) -> impl Responder {
    let result;
    let status_code;

    // Check auth
    match session_id.identity() {
        Some(s_id) => {
            match comment_form.post_id {
                Some(post_id) => {                  
                    // Get user info
                    let db_context = DbContext::new(&pool);
                    match db_context.get_site_user_by_username(&s_id) {
                        Ok(user) => {

                            // Create comment
                            let mut new_comment = NewComment::new();
                            new_comment.site_user_id = user.id;
                            new_comment.post_id = post_id;
                            new_comment.content = String::from(&comment_form.comment_content);

                            match db_context.create_comment(&new_comment) {
                            1 => {
                                    result = "Comment posted.";
                                    status_code = StatusCode::OK;
                                },
                                _ => {
                                    result = "Error creating comment: Database error.";
                                    status_code = StatusCode::INTERNAL_SERVER_ERROR;
                                }
                            }
                        },
                        _ => {
                            result = "Error creating comment: Unable to retrieve user.";
                            status_code = StatusCode::INTERNAL_SERVER_ERROR;
                        } 
                    }          
                },
                _ => {
                    result = "Error creating comment: Invalid Post Id.";
                    status_code = StatusCode::BAD_REQUEST;
                }
            }
        },
        None => {
            result = "Please log in to comment.";
            status_code = StatusCode::FORBIDDEN;
        }
    };
    
    return HttpResponse::build(status_code)
    .content_type("text/html; charset=utf-8")
    .body(result);
}