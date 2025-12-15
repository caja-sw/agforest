use serde::Deserialize;

use crate::constants::{PASSWORD_MAX_BYTE_LENGTH, PASSWORD_MIN_BYTE_LENGTH};

#[derive(Deserialize)]
pub struct Password(String);

impl Password {
    pub fn unwrap(self) -> Result<Vec<u8>, String> {
        let value = self.0.into_bytes();

        let valid = value.iter().all(|c| *c >= 33 && *c <= 126);

        if !valid {
            return Err(
                "비밀번호는 공백과 DELETE를 제외한 ASCII 출력 가능 문자만 포함할 수 있습니다."
                    .to_owned(),
            );
        }

        let len = value.len();

        if len < PASSWORD_MIN_BYTE_LENGTH {
            return Err(format!(
                "비밀번호는 {}글자보다 짧을 수 없습니다.",
                PASSWORD_MIN_BYTE_LENGTH
            ));
        }

        if len > PASSWORD_MAX_BYTE_LENGTH {
            return Err(format!(
                "비밀번호는 {}글자보다 길 수 없습니다.",
                PASSWORD_MAX_BYTE_LENGTH
            ));
        }

        Ok(value)
    }
}
