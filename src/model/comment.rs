use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};
use super::Post;
use super::schema::comment;
use super::super::utility::toolkit;


// Comment //
#[derive(Eq, PartialEq, Debug, Serialize, Deserialize, Queryable, Identifiable, Associations)]
#[belongs_to(Post)]
#[table_name="comment"]
pub struct Comment {
    pub id: i32,
    pub content: String,
    pub post_id: i32,
    pub site_user_id: i32,
    pub parent_comment_id: Option<i32>,
    pub created: DateTime<Utc>,
}

impl Comment {
    pub fn new() -> Self {
        Self {
            id: -1,
            content: String::from(""),
            post_id: -1,
            site_user_id: -1,
            parent_comment_id: None,
            created: Utc::now(),
        }
    }

    pub fn test() -> Self {
        Self {
            id: 7,
            content: String::from("Test"),
            post_id: 7,
            site_user_id: 7,
            parent_comment_id: None,
            created: Utc::now(),
        }
    }
}

// New Comment //
#[derive(Debug, Deserialize, Insertable)]
#[table_name="comment"]
pub struct NewComment {
    pub content: String,
    pub post_id: i32,
    pub site_user_id: i32,
    pub parent_comment_id: Option<i32>,
    pub created: DateTime<Utc>,
}

impl NewComment {
    pub fn new() -> Self {
        Self {
            content: String::from(""),
            post_id: -1,
            site_user_id: -1,
            parent_comment_id: None,
            created: Utc::now(),
        }
    }

    pub fn test() -> Self {
        Self {
            content: String::from("Test"),
            post_id: 7,
            site_user_id: 7,
            parent_comment_id: None,
            created: Utc::now(),
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CommentForm {
    #[serde(deserialize_with = "toolkit::ok_or_none")]
    pub post_id: Option<i32>,
    pub comment_content: String
}