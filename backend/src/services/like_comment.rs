use actix_web::{
    HttpResponse, Responder,
    error::{ErrorInternalServerError, ErrorNotFound},
    post, web,
};
use sqlx::{Pool, Postgres};

#[post("/comments/{id}/like")]
pub async fn like_comment(
    path: web::Path<i32>,
    pool: web::Data<Pool<Postgres>>,
) -> actix_web::Result<impl Responder> {
    let comment_id = path.into_inner();

    sqlx::query!(
        r#"
        UPDATE comments
        SET like_count = like_count + 1
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
