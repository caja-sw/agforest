use std::collections::HashMap;

use serde_json::{Value, json};

use crate::validators::keys::*;

const MIN: usize = 1;
const MAX: usize = 1024;

pub fn validate_comment_content<'k>(
    key: &'k str,
    value: &str,
    constraints: &mut HashMap<&'k str, HashMap<&str, Value>>,
) {
    let mut codes = HashMap::new();
    let length = value.len();

    if length < MIN {
        codes.insert(MIN_LENGTH_CONSTRAINT, json!({ PARAM_MIN: MIN }));
    } else if length > MAX {
        codes.insert(MAX_LENGTH_CONSTRAINT, json!({ PARAM_MAX: MAX }));
    }

    if !codes.is_empty() {
        constraints.insert(key, codes);
    }
}
