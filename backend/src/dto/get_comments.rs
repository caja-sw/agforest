use serde::Serialize;

use crate::{dao::GetCommentsComment, dto::CommentDTO};

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetCommentsResponse {
    pub comments: Vec<CommentDTO>,
}

impl From<Vec<GetCommentsComment>> for GetCommentsResponse {
    fn from(comments: Vec<GetCommentsComment>) -> Self {
        Self {
            comments: comments.into_iter().map(CommentDTO::from).collect(),
        }
    }
}
