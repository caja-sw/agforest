use actix_web::{
    HttpResponse, Responder,
    error::{ErrorInternalServerError, ErrorNotFound},
    get, web,
};
use sqlx::{Pool, Postgres};
use tokio::try_join;

use crate::{
    dto::{Pagination, get_category::Response},
    entity::get_category::{CategoryEntity, PostEntity},
};

#[get("/categories/{id}")]
pub async fn get_category(
    path: web::Path<i32>,
    query: web::Query<Pagination>,
    pool: web::Data<Pool<Postgres>>,
) -> actix_web::Result<impl Responder> {
    let category_id = path.into_inner();
    let (limit, offset) = query.into_inner().destruct();

    let category = sqlx::query_as!(
        CategoryEntity,
        r#"
        SELECT
            id,
            name,
            readonly,
            (
                SELECT COUNT(*) 
                FROM posts
                WHERE category_id = $1 AND deleted_at IS NULL
            ) AS "total_post_count!"
        FROM categories
        WHERE id = $1
        "#,
        category_id
    )
    .fetch_optional(pool.get_ref());

    let posts = sqlx::query_as!(
        PostEntity,
        r#"
        SELECT
            p.id,
            p.author_name,
            p.author_hash,
            p.title,
            p.created_at,
            (SELECT COUNT(*) FROM comments c WHERE c.post_id = p.id AND c.deleted_at IS NULL) AS "comment_count!"
        FROM posts p
        WHERE category_id = $1 AND deleted_at IS NULL
        ORDER BY p.created_at DESC
        LIMIT $2 OFFSET $3
        "#,
        category_id,
        limit,
        offset
    )
    .fetch_all(pool.get_ref());

    let (category, posts) = try_join!(category, posts).map_err(ErrorInternalServerError)?;
    let category = category.ok_or(ErrorNotFound("Category not found"))?;

    Ok(HttpResponse::Ok().json(Response::from((category, posts))))
}
