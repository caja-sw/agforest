use serde::Deserialize;

use crate::vo::{PageLimit, PageOffset};

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Pagination {
    pub limit: PageLimit,
    pub offset: PageOffset,
}

impl Pagination {
    pub fn destruct(self) -> (i64, i64) {
        (self.limit.into(), self.offset.into())
    }
}
