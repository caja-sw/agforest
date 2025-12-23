use serde::Deserialize;
use serde_json::{Value, json};

use crate::constant::{AUTHOR_NAME_MAX_LENGTH, AUTHOR_NAME_MIN_LENGTH};

#[derive(Deserialize)]
pub struct AuthorName(String);

impl AuthorName {
    pub fn unwrap(self) -> Result<String, Value> {
        let value = self.0.trim();
        let len = value.len();

        if len < AUTHOR_NAME_MIN_LENGTH {
            return Err(json!({
                "type": "MIN_LENGTH_CONSTRAINT",
                "value": AUTHOR_NAME_MIN_LENGTH,
            }));
        }

        if len > AUTHOR_NAME_MAX_LENGTH {
            return Err(json!({
                "type": "MAX_LENGTH_CONSTRAINT",
                "value": AUTHOR_NAME_MAX_LENGTH,
            }));
        }

        if value.contains('\n') {
            return Err(json!({
                "type": "LINEBREAK_CONSTRAINT",
            }));
        }

        Ok(value.to_string())
    }
}
