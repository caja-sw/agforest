use serde::Deserialize;

use crate::vo::{AuthorName, CommentContent, Password};

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Request {
    pub author: AuthorName,
    pub password: Password,
    pub content: CommentContent,
}
