use serde::Deserialize;

pub struct PageOffset(i64);

impl PageOffset {
    pub fn new(value: i64) -> Result<Self, String> {
        if value < 0 {
            return Err("page offset must be greater than or equal to 0".to_owned());
        }

        Ok(Self(value))
    }
}

impl<'d> Deserialize<'d> for PageOffset {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'d>,
    {
        let value = i64::deserialize(deserializer)?;
        Self::new(value).map_err(serde::de::Error::custom)
    }
}

impl From<PageOffset> for i64 {
    fn from(value: PageOffset) -> Self {
        value.0
    }
}
