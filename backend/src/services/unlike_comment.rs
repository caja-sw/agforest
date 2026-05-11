use actix_web::{
    HttpResponse, Responder,
    error::{ErrorInternalServerError, ErrorNotFound},
    post, web,
};
use sqlx::{Pool, Postgres};

#[post("/comments/{id}/unlike")]
pub async fn unlike_comment(
    path: web::Path<i32>,
    pool: web::Data<Pool<Postgres>>,
) -> actix_web::Result<impl Responder> {
    let comment_id = path.into_inner();

    sqlx::query!(
        r#"
        UPDATE comments
        SET like_count = GREATEST(like_count - 1, 0)
        WHERE id = $1
        RETURNING id
        "#,
        comment_id
    )
    .fetch_optional(&**pool)
    .await
    .map_err(ErrorInternalServerError)?
    .ok_or(ErrorNotFound("Comment not found"))?;

    Ok(HttpResponse::Ok().finish())
}
