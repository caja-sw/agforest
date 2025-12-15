use serde::Serialize;

use crate::dao::GetCategoriesCategory;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CategoryDTO {
    pub id: i32,
    pub name: String,
}

impl From<GetCategoriesCategory> for CategoryDTO {
    fn from(category: GetCategoriesCategory) -> Self {
        Self {
            id: category.id,
            name: category.name,
        }
    }
}
