// src/models.rs
use serde_derive::Serialize;
use serde_derive::Deserialize;

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub id: u64,
    pub username: String,
}
