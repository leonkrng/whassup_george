use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub sender_email: String,
    pub receiver_email: String,
    pub regexes: Vec<String>
}
