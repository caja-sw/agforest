use serde::Serialize;

use crate::entity::get_categories::CategoryEntity;

use super::Category;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Response {
    pub categories: Vec<Category>,
}

impl From<Vec<CategoryEntity>> for Response {
    fn from(value: Vec<CategoryEntity>) -> Self {
        Self {
            categories: value
                .into_iter()
                .map(|req| Category {
                    id: req.id,
                    name: req.name,
                })
                .collect(),
        }
    }
}
