use actix_web::{
    HttpResponse, Responder,
    error::{ErrorInternalServerError, ErrorNotFound},
    get, web,
};
use sqlx::{Pool, Postgres};

use crate::{
    dto::get_post::Response,
    entity::get_post::{CommentEntity, PostEntity},
};

#[get("/posts/{id}")]
pub async fn get_post(
    path: web::Path<i32>,
    pool: web::Data<Pool<Postgres>>,
) -> actix_web::Result<impl Responder> {
    let post_id = path.into_inner();

    let post = sqlx::query_as!(
        PostEntity,
        r#"
        SELECT 
            p.id,
            p.author_name,
            p.author_hash,
            p.category_id,
            c.name AS category_name,
            p.title,
            p.content,
            p.created_at
        FROM posts p
        INNER JOIN categories c ON p.category_id = c.id
        WHERE p.id = $1
        "#,
        post_id
    )
    .fetch_optional(pool.get_ref())
    .await
    .map_err(ErrorInternalServerError)?
    .ok_or_else(|| ErrorNotFound("Post not found"))?;

    let comments = sqlx::query_as!(
        CommentEntity,
        r#"
        SELECT
            id,
            author_name,
            author_hash,
            content,
            created_at
        FROM comments
        WHERE post_id = $1
        ORDER BY created_at ASC
        "#,
        post_id
    )
    .fetch_all(pool.get_ref())
    .await
    .map_err(ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(Response::from((post, comments))))
}
