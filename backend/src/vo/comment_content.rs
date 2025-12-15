use serde::Deserialize;

use crate::constants::{COMMENT_CONTENT_MAX_LENGTH, COMMENT_CONTENT_MIN_LENGTH};

#[derive(Deserialize)]
pub struct CommentContent(String);

impl CommentContent {
    pub fn unwrap(self) -> Result<String, String> {
        let value = self.0;
        let len = value.len();

        if len < COMMENT_CONTENT_MIN_LENGTH {
            return Err(format!(
                "댓글은 {}글자보다 짧을 수 없습니다.",
                COMMENT_CONTENT_MIN_LENGTH
            ));
        }

        if len > COMMENT_CONTENT_MAX_LENGTH {
            return Err(format!(
                "댓글은 {}글자보다 길 수 없습니다.",
                COMMENT_CONTENT_MAX_LENGTH
            ));
        }

        Ok(value)
    }
}
