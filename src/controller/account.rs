use actix_identity::Identity;
use actix_web::{web, HttpResponse, Responder};
use actix_web::http::StatusCode;
use diesel::pg::PgConnection;
use diesel::{r2d2::ConnectionManager};
use regex::Regex;
use pwhash::bcrypt;
use super::super::model::{NewSiteUser, UserForm, UserLogin};
use super::super::utility::DbContext;

type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

/**
 * Process login request.
 */
pub async fn authenticate(login: web::Form<UserLogin>, session_id: Identity, pool: web::Data<Pool>) -> impl Responder {
    let result;

    // Validate form data
    if !validate_email(&login.email) 
    {
        result = "Please enter a valid email.";
    } 
    else
    {
        if login.password.len() < 1 || login.password.len() > 50
        {
            result = "Please enter a valid password. Length cannot exceed 50 characters.";
        }
        else
        {
            // Get user
            let db_context = DbContext::new(&pool);
            let user = db_context.get_site_user_by_email(login.email());

            // Check if password checks out
            match user {
                Ok(u) => {
                    if bcrypt::verify(&login.password, &u.password)
                    {
                        let session_token = String::from(u.username);
                        session_id.remember(session_token);
                        result = "Logged in!";
                    } 
                    else 
                    {
                        result = "Incorrect password.";
                    }
                },
                Err(_) => {
                    result = "User doesn't exist.";
                }
            }
        }
    }

    return HttpResponse::build(StatusCode::OK)
    .content_type("text/html; charset=utf-8")
    .body(result);
}

/**
 * Get login modal.
 */
pub async fn login() -> impl Responder {
    return HttpResponse::build(StatusCode::OK)
    .content_type("text/html; charset=utf-8")
    .body(include_str!("../view/partial/login.html"));
}

/**
 * Sign user out.
 */
pub async fn logout(session_id: Identity) -> impl Responder {
    session_id.forget();

    return HttpResponse::build(StatusCode::OK)
    .content_type("text/html; charset=utf-8")
    .body("Logged out!");
}

/**
 * Process user signup request.
 */
pub async fn register_user(form: web::Form<UserForm>, session_id: Identity, pool: web::Data<Pool>) -> impl Responder {
    let result;

    // Validate form data	
    if form.username.len() < 1 || form.username.len() > 64
    {
        result = "Please enter a valid username. Length cannot exceed 64 characters.";
    }  
    else
    {
        if !validate_email(&form.email) 
        {
            result = "Please enter a valid email.";
        } 
        else
        {
            if form.password.len() < 1 || form.password.len() > 50
            {
                result = "Please enter a valid password. Length cannot exceed 50 characters.";
            }
            else
            {
                // Make sure user isn't already registred
                let db_context = DbContext::new(&pool);
                if db_context.validate_new_user_email(form.email())
                {
                    // Create new site user  
                    let new_user = NewSiteUser::new(&form);
                    if db_context.create_site_user(new_user) > 0 
                    {        
                        // Create a session for new user   
                        session_id.remember(String::from(form.username()));
                        result = "Registration complete!"
                    }
                    else
                    {
                        result = "Registration failed, please use a different username.";
                    }
                }
                else
                {
                    result = "Registration failed, email already in use.";
                } 
            }
        }
    }
    
    return HttpResponse::build(StatusCode::OK)
    .content_type("text/html; charset=utf-8")
    .body(result);
}

/**
 * Get signup modal.
 */
pub async fn signup() -> impl Responder {
    return HttpResponse::build(StatusCode::OK)
    .content_type("text/html; charset=utf-8")
    .body(include_str!("../view/partial/signup.html"));
}

/**
 * Check email formatting.
 */
fn validate_email(email: &String) -> bool {
    let email_regex = Regex::new(r"^([a-z0-9_+]([a-z0-9_+.]*[a-z0-9_+])?)@([a-z0-9]+([\-\.]{1}[a-z0-9]+)*\.[a-z]{2,6})").unwrap();
    return email_regex.is_match(&email);
}