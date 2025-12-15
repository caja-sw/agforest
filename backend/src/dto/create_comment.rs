use serde::{Deserialize, Serialize};

use crate::vo::{AuthorName, CommentContent, Password};

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateCommentRequest {
    pub author: AuthorName,
    pub password: Password,
    pub content: CommentContent,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateCommentResponse {
    pub id: i32,
}
