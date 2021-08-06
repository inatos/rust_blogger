use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};
use super::schema::post;
use super::super::utility::toolkit;


// Post //
#[derive(Eq, PartialEq, Debug, Serialize, Deserialize, Queryable, Clone, Associations, Identifiable)]
#[table_name="post"]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub label: String,
    pub post_group_id: i32,
    pub post_category_id: i32,
    pub content: String,
    pub embedded: bool,
    pub published: bool, 
    pub created: DateTime<Utc>,
    pub updated: DateTime<Utc>,
}

impl Post {
    pub fn new() -> Self {
        Self {
            id: -1,
            title: String::from(""),
            label: String::from(""),
            post_category_id: -1,
            post_group_id: -1,   
            content: String::from(""),
            embedded: false,
            published: true,
            created: Utc::now(),
            updated: Utc::now(),
        }
    }

    pub fn test() -> Self {
        Self {
            id: 7,
            title: String::from("Test Post"),
            label: String::from(""),
            post_category_id: 0,
            post_group_id: 0,
            content: String::from("<p>Hello there~</p>"),
            embedded: true,
            published: true,
            created: Utc::now(),
            updated: Utc::now(),
        }
    }
}

// New Post //
#[derive(Debug, Deserialize, Insertable)]
#[table_name="post"]
pub struct NewPost {
    pub title: String,
    pub label: String,
    pub post_group_id: i32,
    pub post_category_id: i32,
    pub content: String,
    pub embedded: bool,
    pub published: bool, 
    pub created: DateTime<Utc>,
    pub updated: DateTime<Utc>,
}

impl NewPost {
    pub fn new() -> Self {
        Self {
            title: String::from(""),
            label: String::from(""),
            post_category_id: -1,
            post_group_id: -1,   
            content: String::from(""),
            embedded: false,
            published: true,
            created: Utc::now(),
            updated: Utc::now(),
        }
    }

    pub fn test() -> Self {
        Self {
            title: String::from("Test Post"),
            label: String::from(""),
            post_category_id: 7,
            post_group_id: 7,
            content: String::from("Content"),
            embedded: false,
            published: true,
            created: Utc::now(),
            updated: Utc::now(),
        }
    }
}

// Post Form //
#[derive(Debug, Deserialize, Serialize)]
pub struct PostForm {
    #[serde(deserialize_with = "toolkit::ok_or_none")]
    pub post_id: Option<i32>,
    pub post_title: String,
    pub post_label: String,
    #[serde(deserialize_with = "toolkit::ok_or_none")]
    pub post_group_id:  Option<i32>,
    pub post_group_name: String,
    pub post_category_id: i32,
    pub post_content: String,
    pub post_embedded: bool,
    pub post_published: bool, 
}

// Post Request //
#[derive(Debug, Deserialize)]
pub struct PostRequest {
    pub post_id: Option<i32>,
    pub post: Option<Post>
}