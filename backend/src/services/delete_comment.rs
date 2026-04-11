use actix_web::{
    HttpResponse, Responder, delete,
    error::{ErrorForbidden, ErrorInternalServerError, ErrorNotFound},
    web,
};
use argon2::{Argon2, PasswordHash, PasswordVerifier};
use sqlx::{Pool, Postgres};

use crate::extractors;

#[delete("/comments/{id}")]
pub async fn delete_comment(
    path: web::Path<i32>,
    password: extractors::Password,
    pool: web::Data<Pool<Postgres>>,
) -> actix_web::Result<impl Responder> {
    let comment_id = path.into_inner();

    let mut tx = pool.begin().await.map_err(ErrorInternalServerError)?;

    let comment = sqlx::query!(
        r#"
        SELECT password_hash
        FROM comments
        WHERE id = $1 AND deleted_at IS NULL
        FOR UPDATE
        "#,
        comment_id
    )
    .fetch_optional(&mut *tx)
    .await
    .map_err(ErrorInternalServerError)?
    .ok_or(ErrorNotFound("Comment not found"))?;

    let hash = PasswordHash::new(&comment.password_hash).map_err(ErrorInternalServerError)?;
    let argon2 = Argon2::default();
    argon2
        .verify_password(&password, &hash)
        .map_err(|_| ErrorForbidden("Incorrect password"))?;

    sqlx::query!(
        r#"
        UPDATE comments
        SET deleted_at = now()
        WHERE id = $1
        "#,
        comment_id
    )
    .execute(&mut *tx)
    .await
    .map_err(ErrorInternalServerError)?;

    tx.commit().await.map_err(ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().finish())
}
