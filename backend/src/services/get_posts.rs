use actix_web::{HttpResponse, Responder, error::ErrorInternalServerError, get, web};
use sqlx::{Pool, Postgres};

use crate::{
    dto::{Pagination, get_posts::Response},
    entity::get_posts::PostEntity,
};

#[get("/categories/{id}/posts")]
pub async fn get_posts(
    path: web::Path<i32>,
    query: web::Query<Pagination>,
    pool: web::Data<Pool<Postgres>>,
) -> actix_web::Result<impl Responder> {
    let category_id = path.into_inner();
    let Pagination { offset, limit } = query.into_inner();
    let limit: i64 = limit.into();
    let offset: i64 = offset.into();

    let posts = sqlx::query_as!(
        PostEntity,
        r#"
        WITH paged AS (
            SELECT 
                p.id,
                p.author_name,
                p.author_hash,
                p.title,
                p.created_at,
                COUNT(*) OVER() AS total_count
            FROM posts p
            WHERE p.category_id = $1
            ORDER BY p.created_at DESC
            LIMIT $2 OFFSET $3
        )
        SELECT 
            p.id,
            p.author_name,
            p.author_hash,
            p.title,
            p.created_at,
            p.total_count as "total_count!",
            COUNT(c.id) AS "comment_count!"
        FROM paged p
        LEFT JOIN comments c ON p.id = c.post_id
        GROUP BY p.id, p.author_name, p.author_hash, p.title, p.created_at, p.total_count
        ORDER BY p.created_at DESC
        "#,
        category_id,
        limit,
        offset
    )
    .fetch_all(pool.get_ref())
    .await
    .map_err(ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(Response::from(posts)))
}
