use actix_web::{
    HttpResponse, Responder,
    error::{ErrorInternalServerError, ErrorNotFound},
    get, web,
};
use diesel::{
    dsl::{count_star, exists},
    prelude::*,
    select,
};

use crate::{
    dao::GetPostsPost,
    dto::{GetPostsResponse, Pagination},
    helpers::{DBPool, connect},
};

#[get("/categories/{id}/posts")]
pub async fn get_posts(
    path: web::Path<i32>,
    query: web::Query<Pagination>,
    pool: web::Data<DBPool>,
) -> actix_web::Result<impl Responder> {
    let category_id = path.into_inner();
    let Pagination { offset, limit } = query.into_inner();
    let mut connection = connect(pool)?;

    let data = web::block(move || {
        connection.transaction::<Option<(i64, Vec<GetPostsPost>)>, diesel::result::Error, _>(
            |connection| {
                use crate::schema::categories::dsl::{self as category, categories};
                use crate::schema::posts::dsl::{self as post, posts};

                let total_post_count: i64 = posts
                    .filter(post::category_id.eq(category_id))
                    .select(count_star())
                    .first(connection)?;

                if total_post_count <= 0 {
                    let category_exists: bool =
                        select(exists(categories.filter(category::id.eq(category_id))))
                            .get_result(connection)?;

                    if !category_exists {
                        return Ok(None);
                    }
                }

                let posts_ = posts
                    .inner_join(categories.on(post::category_id.eq(category::id)))
                    .filter(post::category_id.eq(category_id))
                    .order(post::created_at.desc())
                    .offset(offset.into())
                    .limit(limit.into())
                    .select((
                        post::id,
                        post::author_name,
                        post::author_hash,
                        post::category_id,
                        category::name,
                        post::title,
                        post::created_at,
                    ))
                    .load::<GetPostsPost>(connection)?;

                Ok(Some((total_post_count, posts_)))
            },
        )
    })
    .await?
    .map_err(ErrorInternalServerError)?
    .ok_or_else(|| ErrorNotFound("Category not found"))?;

    Ok(HttpResponse::Ok().json(GetPostsResponse::from(data)))
}
