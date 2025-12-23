use serde::{Deserialize, Serialize};

use crate::{
    entity::create_post::PostEntity,
    vo::{AuthorName, Password, PostContent, PostTitle},
};

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Request {
    pub author: AuthorName,
    pub password: Password,
    pub title: PostTitle,
    pub content: PostContent,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Response {
    pub id: i64,
}

impl From<PostEntity> for Response {
    fn from(value: PostEntity) -> Self {
        Self { id: value.id }
    }
}
