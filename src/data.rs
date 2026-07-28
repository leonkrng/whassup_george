use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Data {
    pub last_blog_title: String,
}
