use actix_identity::Identity;
use actix_web::{web, HttpResponse, Responder};
use actix_web::http::StatusCode;
use diesel::pg::PgConnection;
use diesel::{r2d2::ConnectionManager};
use serde::Deserialize;
use super::error::forbidden;
use super::super::model::{Post, NewPost, PostForm, NewPostGroup, SiteUser, EditUserForm};
use super::super::utility::DbContext;
use tera::{Tera, Context};

type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

#[derive(Deserialize)]
pub struct ModeRequest {
   post_id: Option<i32>,
   mode: Option<String>
}

/**
 * Create new post.
 */
pub async fn create_post(form: web::Form<PostForm>, tera: web::Data<Tera>, session_id: Identity, pool: web::Data<Pool>) -> impl Responder {
    let result;
    let mut status_code = StatusCode::INTERNAL_SERVER_ERROR;
 
    // Check for admin
    let is_admin = match session_id.identity() {
        Some(id) => if id == "admin" { true } else { false } 
        None => false,
    };

    // Admin only
    if is_admin
    {  
        let db_context = DbContext::new(&pool);

        // Check if we need to create new group, else validate id
        let post_group_id = match form.post_group_name.is_empty() {
            true => validate_post_group(&db_context, form.post_group_id),
            false => create_post_group(&db_context, &form.post_group_name)
        };

        // Only proceed if we have a valid group
        if post_group_id > 0
        {
            // Set fields for update
            let mut new_post = NewPost::new();
            new_post.title = String::from(&form.post_title);
            new_post.label = String::from(&form.post_label);
            new_post.post_group_id = post_group_id;
            new_post.post_category_id = form.post_category_id;
            new_post.content = String::from(&form.post_content);
            new_post.embedded = form.post_embedded;
            new_post.published = form.post_published;

            // Create post
            match db_context.create_post(new_post) {
                Ok(post) => {
                    // Get post link
                    result = DbContext::get_post_redirect(post.post_category_id).to_string();
                    status_code = StatusCode::OK;
                },
                Err(e) => result = format!("Error creating post. Couldn't create post: {}", e)
            }
        }  
        else
        {
            result = "Error creating post. Invalid Post Group Id.".to_string()
        }  
    }
    else
    {
        // Send them to Gandalf
        return forbidden(tera).await;
    }   
    
    return HttpResponse::build(status_code)
    .content_type("text/html; charset=utf-8")
    .body(result);
}

/**
 * Create new group.
 */
fn create_post_group (db_context: &DbContext, post_group_name: &String) -> i32 {
    return match db_context.create_post_group(NewPostGroup::new(post_group_name)) {
        Ok(post_group) => post_group.id,
        _ => -1
    };
}

/**
 * Delete post.
 */
pub async fn delete_post(form: web::Form<PostForm>, tera: web::Data<Tera>, session_id: Identity, pool: web::Data<Pool>) -> impl Responder {
    let result;
    let mut status_code = StatusCode::INTERNAL_SERVER_ERROR;

    // Check for admin
    let is_admin = match session_id.identity() {
        Some(id) => if id == "admin" { true } else { false } 
        None => false,
    };
  
    // Admin only
    if is_admin
    {     
        // Validate post id
        match form.post_id {
            Some(post_id) if post_id > 0 => {
                let db_context = DbContext::new(&pool);

                // Delete post
                match db_context.delete_post(post_id) {
                    Ok(_) => {
                        result = "/editor".to_string();
                        status_code = StatusCode::OK;
                    },
                    Err(e) => result = format!("Error deleting post: {}", e)
                } 
            },
            _ => result = "Error deleting post. Bad Post Id.".to_string()
        };
    }
    else
    {
        // Send them to Gandalf
        return forbidden(tera).await;
    }   
    
    return HttpResponse::build(status_code)
    .content_type("text/html; charset=utf-8")
    .body(result);
}

/**
 * Edit post.
 */
pub async fn edit_post(form: web::Form<PostForm>, tera: web::Data<Tera>, session_id: Identity, pool: web::Data<Pool>) -> impl Responder {
    let result;
    let mut status_code = StatusCode::INTERNAL_SERVER_ERROR;

    // Check for admin
    let is_admin = match session_id.identity() {
        Some(id) => if id == "admin" { true } else { false } 
        None => false,
    };
  
    // Admin only
    if is_admin
    {     
        // Validate post id
        match form.post_id {
            Some(post_id) if post_id > 0 => {
                let db_context = DbContext::new(&pool);

                // Make sure post exists
                match db_context.get_post(post_id) {
                    Ok(mut post) => {
                        // Check if we need to create new group, else validate id
                        let post_group_id = match form.post_group_name.is_empty() {
                            true => validate_post_group(&db_context, form.post_group_id),
                            false => create_post_group(&db_context, &form.post_group_name)
                        };

                        // Only proceed if we have a valid group
                        if post_group_id > 0 
                        {
                            // Set fields for update
                            post.title = String::from(&form.post_title);
                            post.label = String::from(&form.post_label);
                            post.post_group_id = post_group_id;
                            post.post_category_id = form.post_category_id;
                            post.content = String::from(&form.post_content);
                            post.embedded = form.post_embedded;
                            post.published = form.post_published;

                            // Edit post
                            match db_context.update_post(post) {
                                Ok(p) => {
                                    // Get post link
                                    result = DbContext::get_post_redirect(p.post_category_id).to_string();
                                    status_code = StatusCode::OK;
                                },
                                Err(e) => result = format!("Error editing post. Couldn't update post: {}", e)
                            } 
                        }
                        else
                        {
                            result = "Error editing post. Unable to create Post Group".to_string();
                        }
                    },
                    Err(e) => result = format!("Error editing post. Unable to retrieve post: {}", e)
                }    
            },
            _ => result = "Error editing post. Bad Post Id.".to_string()
        };
    }
    else
    {
        // Send them to Gandalf
        return forbidden(tera).await;
    }   
    
    return HttpResponse::build(status_code)
    .content_type("text/html; charset=utf-8")
    .body(result);
}

/**
 * Edit user.
 */
pub async fn edit_user(form: web::Form<EditUserForm>, tera: web::Data<Tera>, session_id: Identity, pool: web::Data<Pool>) -> impl Responder {
    let result;
    let mut status_code = StatusCode::INTERNAL_SERVER_ERROR;

    // Check for admin
    let is_admin = match session_id.identity() {
        Some(id) => if id == "admin" { true } else { false } 
        None => false,
    };
  
    // Admin only
    if is_admin
    {     
        // Validate user id
        if form.id > 0 
        {
            let db_context = DbContext::new(&pool);

            // Make sure user exists
            match db_context.get_site_user(form.id) {
                Ok(mut user) => {
                    // Set fields for update
                    user.username = String::from(&form.username);
                    user.email = String::from(&form.email);
                    user.site_user_status_id = form.site_user_status_id;

                    // Edit user
                    match db_context.update_site_user(user) {
                        Ok(u) => {
                            result = u.username;
                            status_code = StatusCode::OK;
                        },
                        Err(e) => result = format!("Error editing user. Couldn't update user: {}", e)
                    } 
                },
                Err(e) => result = format!("Error editing user. Unable to retrieve user: {}", e)
            }    
        }
        else
        {
            result = "Error editing user. Bad User Id.".to_string()
        }
    }
    else
    {
        // Send them to Gandalf
        return forbidden(tera).await;
    }  
    
    return HttpResponse::build(status_code)
    .content_type("text/html; charset=utf-8")
    .body(result);
}

/**
 * Display post editor.
 */
pub async fn editor(request: web::Query<ModeRequest>, tera: web::Data<Tera>, session_id: Identity, pool: web::Data<Pool>) -> impl Responder {
    let is_admin = match session_id.identity() {
        Some(id) => if id == "admin" { true } else { false } 
        None => false,
    };
    
    // Only proceed if admin
    if is_admin
    {
        let mut data = Context::new();
        data.insert("title", "Editor");

        // Get Posts
        let db_context = DbContext::new(&pool);
        let posts: Vec<Post> = match db_context.get_posts() {
            Ok(p) => p,
            _ => Vec::new()
        };
        data.insert("posts", &posts);

        // Get Post Categories
        let categories = db_context.get_post_categories();
        data.insert("categories", &categories);

        // Get Post Groups
        let groups = db_context.get_post_groups();
        data.insert("groups", &groups);

        // Check if we're deleting/editing a post
        match request.post_id {
            Some(id) => {  
                match db_context.get_post(id) {
                    Ok(p) => {
                        // Make sure data is valid
                        if categories.contains_key(&p.post_category_id) && groups.contains_key(&p.post_group_id)
                        {
                            data.insert("post", &p);
                            data.insert("category", &categories[&p.post_category_id]);
                            data.insert("group", &groups[&p.post_group_id]);

                            let editor_mode = match &request.mode {
                                Some(mode) => mode,
                                _ => "edit"
                            };
                            data.insert("editor_mode", &editor_mode);
                        }
                    },
                    _ => ()
                }
            },
            _ => ()
        }
        
        // Render view
        let rendered = tera.render("editor.html", &data).unwrap();
        return HttpResponse::Ok().body(rendered);
    }
    else
    {
        // Send them to Gandalf
        return forbidden(tera).await;
    }   
}

/**
 * Display users.
 */
pub async fn users(tera: web::Data<Tera>, session_id: Identity, pool: web::Data<Pool>) -> impl Responder {
    let is_admin = match session_id.identity() {
        Some(id) => if id == "admin" { true } else { false } 
        None => false,
    };
    
    // Only proceed if admin
    if is_admin
    {
        let mut data = Context::new();
        data.insert("title", "Users");

        // Get User Statuses
        let db_context = DbContext::new(&pool);
        let statuses = db_context.get_site_user_statuses();
        data.insert("statuses", &statuses);

        // Get Users       
        let users: Vec<SiteUser> = match db_context.get_site_users() {
            Ok(u) => u,
            _ => Vec::new()
        };
        data.insert("users", &users);
        
        // Render view
        let rendered = tera.render("users.html", &data).unwrap();
        return HttpResponse::Ok().body(rendered);
    }
    else
    {
        // Send them to Gandalf
        return forbidden(tera).await;
    }   
}

/**
 * Check for valid Group Id.
 */
fn validate_post_group (db_context: &DbContext, post_group_id: Option<i32>) -> i32 {
    return match post_group_id {
        Some(id) => match db_context.get_post_group(id) {
            Ok(post_group) => post_group.id,
            _ => -1
        },
        _ => -1
    };
}