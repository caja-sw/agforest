use serde::Deserialize;

use crate::constants::{AUTHOR_NAME_MAX_LENGTH, AUTHOR_NAME_MIN_LENGTH};

#[derive(Deserialize)]
pub struct AuthorName(String);

impl AuthorName {
    pub fn unwrap(self) -> Result<String, String> {
        let value = self.0.trim();
        let len = value.len();

        if len < AUTHOR_NAME_MIN_LENGTH {
            return Err(format!(
                "작성자명은 {}글자보다 짧을 수 없습니다.",
                AUTHOR_NAME_MIN_LENGTH
            ));
        }

        if len > AUTHOR_NAME_MAX_LENGTH {
            return Err(format!(
                "작성자명은 {}글자보다 길 수 없습니다.",
                AUTHOR_NAME_MAX_LENGTH
            ));
        }

        if value.contains('\n') {
            return Err("작성자명은 줄바꿈을 포함할 수 없습니다.".to_string());
        }

        Ok(value.to_string())
    }
}
