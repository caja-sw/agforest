use serde::Serialize;

use crate::{dao::GetPostsPost, dto::CompactPostDTO};

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetPostsResponse {
    pub total_post_count: i64,
    pub posts: Vec<CompactPostDTO>,
}

impl From<(i64, Vec<GetPostsPost>)> for GetPostsResponse {
    fn from((total_post_count, posts): (i64, Vec<GetPostsPost>)) -> Self {
        Self {
            total_post_count,
            posts: posts.into_iter().map(CompactPostDTO::from).collect(),
        }
    }
}
