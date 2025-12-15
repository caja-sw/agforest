use serde::Deserialize;

use crate::vo::{PageLimit, PageOffset};

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Pagination {
    pub offset: PageOffset,
    pub limit: PageLimit,
}
