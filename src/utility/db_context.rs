#![allow(dead_code)]
use actix_web::web;
use chrono::Utc;
use diesel::prelude::*;
use diesel::pg::PgConnection;
use diesel::{OptionalExtension, r2d2::ConnectionManager};
use num_traits::FromPrimitive;
use std::collections::HashMap;
use super::super::model::*;
use super::super::model::comment::{Comment, NewComment};
use super::super::model::schema::comment::dsl::*;
use super::super::model::post::{Post, NewPost};
use super::super::model::schema::post::dsl::*;
use super::super::model::post_category::PostCategory;
use super::super::model::schema::post_category::dsl::*;
use super::super::model::post_group::{PostGroup, NewPostGroup};
use super::super::model::schema::post_group::dsl::*;
use super::super::model::site_user::{SiteUser, NewSiteUser};
use super::super::model::schema::site_user::dsl::*;
use super::super::model::site_user_status::SiteUserStatus;
use super::super::model::schema::site_user_status::dsl::*;
type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub struct DbContext {
    connection: r2d2::PooledConnection<diesel::r2d2::ConnectionManager<diesel::PgConnection>>,
}

impl DbContext {
    pub fn new(pool: &web::Data<Pool>) -> Self {
        return Self {
            connection: pool.get().unwrap(),
        };  
    }

    pub fn create_comment(&self, new_comment: &NewComment) -> usize {
        return diesel::insert_into(schema::comment::table)
            .values(new_comment)
            .execute(&self.connection)
            .expect("Error creating comment.");
    }

    pub fn create_post(&self, new_post: NewPost) ->  Result<Post, diesel::result::Error> {
        return diesel::insert_into(schema::post::table)
            .values(&new_post)
            .get_result(&self.connection);
    }

    pub fn create_post_group(&self, new_post_group: NewPostGroup) -> Result<PostGroup, diesel::result::Error> {
        return diesel::insert_into(schema::post_group::table)
            .values(&new_post_group)
            .get_result(&self.connection);
    }

    pub fn create_site_user(&self, new_site_user: NewSiteUser) -> usize {  
        return diesel::insert_into(schema::site_user::table)
            .values(&new_site_user)
            .execute(&self.connection)
            .expect("Error creating user.");
    }

    pub fn delete_post(&self, p_id: i32) -> Result<i32, diesel::result::Error>
    {
        return diesel::delete(post.find(p_id))
            .returning(schema::post::id)
            .get_result(&self.connection);
    }

    pub fn get_post(&self, p_id: i32) -> Result<Post, diesel::result::Error> {
        return post
            .find(p_id)
            .first(&self.connection); 
    }

    pub fn get_post_categories(&self) -> HashMap<i32, String> {
        let categories = post_category
            .load::<PostCategory>(&self.connection)
            .expect("Error getting post categories.");
        
        return categories.iter().map(|c| (c.id, String::from(&c.name))).collect();
    }

    pub fn get_post_comments(&self, p_id: i32) -> Vec<Comment> {
        return comment
            .filter(post_id.eq(p_id))
            .load::<Comment>(&self.connection)
            .expect("Error getting post comments."); 
    }

    pub fn get_post_group(&self, group_id: i32) -> Result<PostGroup, diesel::result::Error> {
        return post_group 
            .find(group_id)
            .first(&self.connection); 
    }

    pub fn get_post_group_by_name(&self, post_group_name: &String) -> Result<PostGroup, diesel::result::Error> {
        return post_group 
            .filter(schema::post_group::name.eq(post_group_name))
            .get_result::<PostGroup>(&self.connection)
    }

    pub fn get_post_groups(&self) -> HashMap<i32, String> {
        let groups = post_group
            .load::<PostGroup>(&self.connection)
            .expect("Error getting post groups.");
        
        return groups.iter().map(|c| (c.id, String::from(&c.name))).collect();
    }

    pub fn get_post_latest(&self) -> Option<Post> {
        // Get latest post
        return match post
            .filter(published)
            .order(schema::post::id.desc())
            .first::<Post>(&self.connection) {
                Ok(p) => Some(p),
                _ => None
        };
    }

    /**
     * Get redirect path.
     */
    pub fn get_post_redirect (category_id: i32) -> &'static str {
        return match FromPrimitive::from_i32(category_id) {
            Some(Category::Blog) => "/blog",
            Some(Category::Git) => "/git",
            _ => "/home" 
        };
    }

    pub fn get_posts(&self) -> Result<Vec<Post>, diesel::result::Error> {
        return post
            .load::<Post>(&self.connection);
    }

    pub fn get_posts_by_category(&self, category_id: i32) -> Vec<Post> {
        return post
            .filter(post_category_id.eq(category_id))
            .filter(published)
            .order(schema::post::id.desc())
            .load::<Post>(&self.connection)
            .expect("Error getting posts."); 
    }

    /**
     * Get posts w/ comments & their users.
     */
    pub fn get_posts_info(&self, category_id: i32) -> Vec<(Post, Vec<(Comment, SiteUser)>)> {  
        let posts_info: Vec<(Post, Vec<(Comment, SiteUser)>)>;

        // Get posts by category
        let posts = self.get_posts_by_category(category_id);

        // Get comments w/ user info
        let comments_info: Vec<(Comment, SiteUser)> = Comment::belonging_to(&posts)
        .inner_join(site_user.on(site_user_status_id.ne(3))) // Only get users that aren't banned
        .load(&self.connection)
        .expect("Error getting post info.");

        // Create a wrapper vec to store comment info
        let mut c_info_wrapper: Vec<Vec<(Comment, SiteUser)>> = Vec::new();
        c_info_wrapper.push(comments_info);

        // Pair comment info w/ their parent post
        posts_info = posts.into_iter().zip(c_info_wrapper).collect();
        return posts_info;
    }

    pub fn get_site_user(&self, s_id: i32) -> Result<SiteUser, diesel::result::Error> {
        return site_user
            .find(s_id)
            .first(&self.connection); 
    }

    pub fn get_site_users(&self) -> Result<Vec<SiteUser>, diesel::result::Error> {
        return site_user
            .load::<SiteUser>(&self.connection);
    }

    pub fn get_site_user_by_email(&self, user_email: &String) -> Result<SiteUser, diesel::result::Error> {
        return site_user
            .filter(email.eq(user_email))
            .first::<SiteUser>(&self.connection);
    }

    pub fn get_site_user_by_username(&self, u_name: &String) -> Result<SiteUser, diesel::result::Error> {    
        return site_user
            .filter(username.eq(u_name))
            .first::<SiteUser>(&self.connection);
    }

    pub fn get_site_user_status(&self, user_status_id: i32) -> SiteUserStatus {
        return site_user_status
            .find(user_status_id)
            .first(&self.connection)
            .expect("Error getting site user status."); 
    }

    pub fn get_site_user_statuses(&self) -> HashMap<i32, String> {
        let statuses = site_user_status
            .load::<SiteUserStatus>(&self.connection)
            .expect("Error getting site user statuses.");
        
        return statuses.iter().map(|c| (c.id, String::from(&c.name))).collect();
    }

    pub fn update_post(&self, p: Post) -> Result<Post, diesel::result::Error> {
        return diesel::update(post.find(p.id))
            .set((
                schema::post::post_category_id.eq(p.post_category_id),
                schema::post::post_group_id.eq(p.post_group_id),
                schema::post::title.eq(p.title),
                schema::post::content.eq(p.content),
                schema::post::embedded.eq(p.embedded),
                schema::post::published.eq(p.published),
                schema::post::updated.eq(Utc::now())
            ))
            .get_result::<Post>(&self.connection);
    }

    pub fn update_site_user(&self, s: SiteUser) -> Result<SiteUser, diesel::result::Error> {
        return diesel::update(site_user.find(s.id))
            .set((           
                schema::site_user::username.eq(s.username),
                schema::site_user::email.eq(s.email),
                schema::site_user::site_user_status_id.eq(s.site_user_status_id),
                schema::site_user::updated.eq(Utc::now())
            ))
            .get_result::<SiteUser>(&self.connection);
    }

    pub fn validate_new_user_email(&self, user_email: &String) -> bool {
        return match site_user 
            .filter(schema::site_user::email.eq(user_email))
            .get_result::<SiteUser>(&self.connection)
            .optional() {
                Ok(None) => true,
                _ => false,
            };
    }
}