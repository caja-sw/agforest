use serde::Deserialize;
use serde_json::{Value, json};

use crate::constant::{POST_TITLE_MAX_LENGTH, POST_TITLE_MIN_LENGTH};

#[derive(Deserialize)]
pub struct PostTitle(String);

impl PostTitle {
    pub fn unwrap(self) -> Result<String, Value> {
        let value = self.0.trim();
        let len = value.len();

        if len < POST_TITLE_MIN_LENGTH {
            return Err(json!({
                "type": "MIN_LENGTH_CONSTRAINT",
                "value": POST_TITLE_MIN_LENGTH,
            }));
        }

        if len > POST_TITLE_MAX_LENGTH {
            return Err(json!({
                "type": "MAX_LENGTH_CONSTRAINT",
                "value": POST_TITLE_MAX_LENGTH,
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
