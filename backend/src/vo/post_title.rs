use serde::Deserialize;

use crate::constants::{POST_TITLE_MAX_LENGTH, POST_TITLE_MIN_LENGTH};

#[derive(Deserialize)]
pub struct PostTitle(String);

impl PostTitle {
    pub fn unwrap(self) -> Result<String, String> {
        let value = self.0.trim();
        let len = value.len();

        if len < POST_TITLE_MIN_LENGTH {
            return Err(format!(
                "게시글 제목은 {}글자보다 짧을 수 없습니다.",
                POST_TITLE_MIN_LENGTH
            ));
        }

        if len > POST_TITLE_MAX_LENGTH {
            return Err(format!(
                "게시글 제목은 {}글자보다 길 수 없습니다.",
                POST_TITLE_MAX_LENGTH
            ));
        }

        if value.contains('\n') {
            return Err("게시글 제목은 줄바꿈을 포함할 수 없습니다.".to_string());
        }

        Ok(value.to_string())
    }
}
