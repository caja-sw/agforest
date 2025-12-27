use serde::Serialize;

use crate::entity::get_categories::CategoryEntity;

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
                .map(|category| Category {
                    id: category.id,
                    name: category.name,
                    readonly: category.readonly,
                })
                .collect(),
        }
    }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Category {
    pub id: i64,
    pub name: String,
    pub readonly: bool,
}
