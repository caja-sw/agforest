use serde::Deserialize;

use crate::constant::{PAGE_LIMIT_MAX, PAGE_LIMIT_MIN};

pub struct PageLimit(i64);

impl PageLimit {
    pub fn new(value: i64) -> Result<Self, String> {
        if value < PAGE_LIMIT_MIN {
            return Err(format!(
                "page limit must be greater than {}",
                PAGE_LIMIT_MIN
            ));
        }

        if value > PAGE_LIMIT_MAX {
            return Err(format!("page limit must be less than {}", PAGE_LIMIT_MAX));
        }

        Ok(Self(value))
    }
}

impl<'d> Deserialize<'d> for PageLimit {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'d>,
    {
        let value = i64::deserialize(deserializer)?;
        Self::new(value).map_err(serde::de::Error::custom)
    }
}

impl From<PageLimit> for i64 {
    fn from(value: PageLimit) -> Self {
        value.0
    }
}
