use super::schema::post_group;
use serde::{Serialize, Deserialize};

// Post Group //
#[derive(Eq, PartialEq, Debug, Serialize, Deserialize, Queryable)]
pub struct PostGroup {
    pub id: i32,
    pub name: String,
}

impl PostGroup {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self {
            id: -1,
            name: String::from(""),
        }
    }
}

// New Post Group //
#[derive(Debug, Deserialize, Insertable)]
#[table_name="post_group"]
pub struct NewPostGroup {
    pub name: String,
}

impl NewPostGroup {
    pub fn new(post_group_name: &String) -> Self {
        Self {
            name: String::from(post_group_name),
        }
    }
}