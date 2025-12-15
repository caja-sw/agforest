use serde::{Deserialize, Serialize};

use crate::vo::{AuthorName, Password, PostContent, PostTitle};

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreatePostRequest {
    pub author: AuthorName,
    pub password: Password,
    pub title: PostTitle,
    pub content: PostContent,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreatePostResponse {
    pub id: i32,
}
