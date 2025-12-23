use serde::Deserialize;

#[derive(Deserialize)]
pub struct CommentEntity {
    pub password_hash: String,
}
