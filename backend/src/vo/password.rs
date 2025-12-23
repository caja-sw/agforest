use serde::Deserialize;
use serde_json::{Value, json};

use crate::constant::{PASSWORD_MAX_BYTE_LENGTH, PASSWORD_MIN_BYTE_LENGTH};

#[derive(Deserialize)]
pub struct Password(String);

impl Password {
    pub fn unwrap(self) -> Result<Vec<u8>, Value> {
        let value = self.0.into_bytes();

        let valid = value.iter().all(|c| *c >= 33 && *c <= 126);

        if !valid {
            return Err(json!({
                "type": "CHARACTER_CONSTRAINT",
            }));
        }

        let len = value.len();

        if len < PASSWORD_MIN_BYTE_LENGTH {
            return Err(json!({
                "type": "MIN_LENGTH_CONSTRAINT",
                "value": PASSWORD_MIN_BYTE_LENGTH,
            }));
        }

        if len > PASSWORD_MAX_BYTE_LENGTH {
            return Err(json!({
                "type": "MAX_LENGTH_CONSTRAINT",
                "value": PASSWORD_MAX_BYTE_LENGTH,
            }));
        }

        Ok(value)
    }
}
