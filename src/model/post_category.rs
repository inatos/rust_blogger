use num_derive::FromPrimitive;  
use serde::{Serialize, Deserialize};

// Post Category //
#[derive(Eq, PartialEq, Debug, Serialize, Deserialize, Queryable)]
pub struct PostCategory {
    pub id: i32,
    pub name: String,
}

impl PostCategory {
    pub fn new() -> Self {
        Self {
            id: -1,
            name: String::from(""),
        }
    }

    pub fn test() -> Self {
        Self {
            id: 0,
            name: String::from("blog"),
        }
    }
}

impl From<PostCategory> for (i32, String) {
    fn from(category: PostCategory) -> (i32, String) {
        let PostCategory {id, name} = category;
        return (id, name);
    }
}

// Helper Enum //
#[derive(FromPrimitive)]
pub enum Category {
    Blog,
    Git
}