use serde::{Deserialize, Serialize};

use crate::{
    entity::create_comment::CommentEntity,
    vo::{AuthorName, CommentContent, Password},
};

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Request {
    pub author: AuthorName,
    pub password: Password,
    pub content: CommentContent,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Response {
    pub id: i64,
}

impl From<CommentEntity> for Response {
    fn from(value: CommentEntity) -> Self {
        Self { id: value.id }
    }
}
