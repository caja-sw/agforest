use actix_web::{
    HttpResponse, Responder,
    error::{ErrorInternalServerError, ErrorNotFound},
    post, web,
};
use sqlx::{Pool, Postgres};

#[post("/posts/{id}/unlike")]
pub async fn unlike_post(
    path: web::Path<i32>,
    pool: web::Data<Pool<Postgres>>,
) -> actix_web::Result<impl Responder> {
    let post_id = path.into_inner();

    sqlx::query!(
        r#"
        UPDATE posts
        SET like_count = GREATEST(like_count - 1, 0)
        WHERE id = $1 AND deleted_at IS NULL
        RETURNING id
        "#,
        post_id
    )
    .fetch_optional(&**pool)
    .await
    .map_err(ErrorInternalServerError)?
    .ok_or(ErrorNotFound("Post not found"))?;

    Ok(HttpResponse::Ok().finish())
}
