use serde::Deserialize;
use serde_json::{Value, json};

use crate::constant::{COMMENT_CONTENT_MAX_LENGTH, COMMENT_CONTENT_MIN_LENGTH};

#[derive(Deserialize)]
pub struct CommentContent(String);

impl CommentContent {
    pub fn unwrap(self) -> Result<String, Value> {
        let value = self.0;
        let len = value.len();

        if len < COMMENT_CONTENT_MIN_LENGTH {
            return Err(json!({
                "type": "MIN_LENGTH_CONSTRAINT",
                "value": COMMENT_CONTENT_MIN_LENGTH,
            }));
        }

        if len > COMMENT_CONTENT_MAX_LENGTH {
            return Err(json!({
                "type": "MAX_LENGTH_CONSTRAINT",
                "value": COMMENT_CONTENT_MAX_LENGTH,
            }));
        }

        Ok(value)
    }
}
