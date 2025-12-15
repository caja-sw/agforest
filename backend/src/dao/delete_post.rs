use diesel::prelude::*;

#[derive(Queryable)]
#[diesel(table_name = crate::schema::posts)]
pub struct DeletePostPost {
    pub password_hash: String,
}
