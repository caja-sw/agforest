use diesel::prelude::*;

#[derive(Queryable)]
#[diesel(table_name = crate::schema::categories)]
pub struct GetCategoriesCategory {
    pub id: i32,
    pub name: String,
}
