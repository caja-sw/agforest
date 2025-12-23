use serde::Deserialize;

#[derive(Deserialize)]
pub struct PostEntity {
    pub password_hash: String,
}
