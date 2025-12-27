use actix_web::{
    HttpRequest, HttpResponse, Responder, delete,
    error::{ErrorForbidden, ErrorInternalServerError, ErrorNotFound},
    web,
};
use argon2::{Argon2, PasswordHash, PasswordVerifier};
use sqlx::{Pool, Postgres};

use crate::{entity::delete_post::PostEntity, helper::get_password};

#[delete("/posts/{id}")]
pub async fn delete_post(
    req: HttpRequest,
    path: web::Path<i32>,
    pool: web::Data<Pool<Postgres>>,
) -> actix_web::Result<impl Responder> {
    let password = get_password(&req)?.to_owned();
    let post_id = path.into_inner();
    let mut tx = pool.begin().await.map_err(ErrorInternalServerError)?;

    let post = sqlx::query_as!(
        PostEntity,
        r#"
        SELECT
            p.password_hash
        FROM posts p
        JOIN categories c ON p.category_id = c.id
        WHERE p.id = $1 AND p.deleted_at IS NULL AND c.readonly = false 
        FOR UPDATE OF p;
        "#,
        post_id
    )
    .fetch_optional(&mut *tx)
    .await
    .map_err(ErrorInternalServerError)?
    .ok_or(ErrorNotFound("Post not found"))?;

    let hash = PasswordHash::new(&post.password_hash).map_err(ErrorInternalServerError)?;
    let argon2 = Argon2::default();
    argon2
        .verify_password(&password, &hash)
        .map_err(|_| ErrorForbidden("Incorrect password"))?;

    sqlx::query!(
        r#"
        UPDATE posts
        SET deleted_at = NOW()
        WHERE id = $1
        "#,
        post_id
    )
    .execute(&mut *tx)
    .await
    .map_err(ErrorInternalServerError)?;

    tx.commit().await.map_err(ErrorInternalServerError)?;

    Ok(HttpResponse::NoContent())
}
