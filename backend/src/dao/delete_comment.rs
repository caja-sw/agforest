use diesel::prelude::*;

#[derive(Queryable)]
#[diesel(table_name = crate::schema::comments)]
pub struct DeleteCommentComment {
    pub password_hash: String,
}
