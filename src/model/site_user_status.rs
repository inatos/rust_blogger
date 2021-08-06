use serde::{Serialize, Deserialize};

// Site User Status //
#[derive(Eq, PartialEq, Debug, Serialize, Deserialize, Queryable)]
pub struct SiteUserStatus {
    pub id: i32,
    pub name: String,
}

impl SiteUserStatus {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self {
            id: -1,
            name: String::from(""),
        }
    }
}