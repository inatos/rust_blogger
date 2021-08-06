use actix_web::web;
use chrono::{DateTime, Utc};
use pwhash::bcrypt;
use serde::{Serialize, Deserialize};
use super::schema::site_user;

/// Site User ///
#[derive(Eq, PartialEq, Debug, Serialize, Deserialize, Queryable)]
pub struct SiteUser {
    pub id: i32,
    pub site_user_status_id: i32,
    pub username: String,
    pub email: String,
    pub password: String,
    pub created: DateTime<Utc>,
    pub updated: DateTime<Utc>,
}

impl SiteUser {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self {
            id: -1,
            site_user_status_id: 0,
            username: String::from(""),
            email: String::from(""),
            password: String::from(""),
            created: Utc::now(),
            updated: Utc::now(),
        }
    }
}

/// New Site User ///
#[derive(Debug, Deserialize, Insertable)]
#[table_name="site_user"]
pub struct NewSiteUser {
    pub site_user_status_id: i32,
    pub username: String, 
    pub email: String,
    pub password: String,
    pub created: DateTime<Utc>,
    pub updated: DateTime<Utc>,
}

impl NewSiteUser {
    pub fn new(form: &web::Form<UserForm>) -> Self {
        // Hash pwd
        let hash = bcrypt::hash(form.password.to_string()).expect("Hash cannot be generated.");

        // Set fields
        Self {
            site_user_status_id: 1,
            username: form.username.to_string(),
            email: form.email.to_string(),
            password: hash,
            created: Utc::now(),
            updated: Utc::now(),
        }
    }
}

/// User Form ///
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct UserForm {
    pub username: String,
    pub email: String,
    pub password: String
}

impl UserForm {
    pub fn username(&self) -> &String {
        &self.username
    }

    pub fn email(&self) -> &String {
        &self.email
    }
}

/// Edit User Form ///
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct EditUserForm {
    pub id: i32,
    pub site_user_status_id: i32,
    pub username: String,
    pub email: String
}

/// User Login ///
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct UserLogin {
    pub email: String,
    pub password: String,
}

impl UserLogin {
    pub fn email(&self) -> &String {
        &self.email
    }
}