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
    let (limit, offset) = query.into_inner().destruct();

    let posts = sqlx::query_as!(
        PostEntity,
        r#"
        WITH paged AS (
            SELECT 
                COUNT(*) OVER() AS total_count,
                p.id,
                p.author_name,
                p.author_hash,
                p.title,
                p.created_at
            FROM posts p
            WHERE p.category_id = $1
            ORDER BY p.created_at DESC
            LIMIT $2 OFFSET $3
        )
        SELECT 
            p.total_count AS "total_count!",
            p.id,
            p.author_name,
            p.author_hash,
            p.title,
            p.created_at,
            COUNT(c.id) AS "comment_count!"
        FROM paged p
        LEFT JOIN comments c ON p.id = c.post_id
        GROUP BY p.total_count, p.id, p.author_name, p.author_hash, p.title, p.created_at
        ORDER BY p.created_at DESC
        "#,
        category_id,
        limit,
        offset
    )
    .fetch_all(pool.get_ref())
    .await
    .map_err(ErrorInternalServerError)?;

    let total_count = if let Some(post) = posts.first() {
        post.total_count
    } else {
        sqlx::query!(
            r#"
                SELECT
                    COUNT(*) as "total_count!"
                FROM posts
                WHERE category_id = $1
                "#,
            category_id
        )
        .fetch_one(pool.get_ref())
        .await
        .map_err(ErrorInternalServerError)?
        .total_count
    };

    Ok(HttpResponse::Ok().json(Response::from((total_count, posts))))
}
