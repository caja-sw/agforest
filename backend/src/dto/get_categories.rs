use serde::Serialize;

use crate::{dao::GetCategoriesCategory, dto::CategoryDTO};

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetCategoriesResponse {
    pub categories: Vec<CategoryDTO>,
}

impl From<Vec<GetCategoriesCategory>> for GetCategoriesResponse {
    fn from(categories: Vec<GetCategoriesCategory>) -> Self {
        Self {
            categories: categories.into_iter().map(CategoryDTO::from).collect(),
        }
    }
}
