use serde::Deserialize;

use crate::constants::{POST_CONTENT_MAX_LENGTH, POST_CONTENT_MIN_LENGTH};

#[derive(Deserialize)]
pub struct PostContent(String);

impl PostContent {
    pub fn unwrap(self) -> Result<String, String> {
        let value = self.0;
        let len = value.len();

        if len < POST_CONTENT_MIN_LENGTH {
            return Err(format!(
                "게시글은 {}글자보다 짧을 수 없습니다.",
                POST_CONTENT_MIN_LENGTH
            ));
        }

        if len > POST_CONTENT_MAX_LENGTH {
            return Err(format!(
                "게시글은 {}글자보다 길 수 없습니다.",
                POST_CONTENT_MAX_LENGTH
            ));
        }

        Ok(value)
    }
}
