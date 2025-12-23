use serde::Deserialize;

#[derive(Deserialize)]
pub struct CategoryEntity {
    pub id: i64,
    pub name: String,
}
