use serde::Deserialize;
use serde_json::{Value, json};

use crate::constant::{POST_CONTENT_MAX_LENGTH, POST_CONTENT_MIN_LENGTH};

#[derive(Deserialize)]
pub struct PostContent(String);

impl PostContent {
    pub fn unwrap(self) -> Result<String, Value> {
        let value = self.0;
        let len = value.len();

        if len < POST_CONTENT_MIN_LENGTH {
            return Err(json!({
                "type": "MIN_LENGTH_CONSTRAINT",
                "value": POST_CONTENT_MIN_LENGTH,
            }));
        }

        if len > POST_CONTENT_MAX_LENGTH {
            return Err(json!({
                "type": "MAX_LENGTH_CONSTRAINT",
                "value": POST_CONTENT_MAX_LENGTH,
            }));
        }

        Ok(value)
    }
}
