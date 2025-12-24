use std::collections::HashMap;

use actix_web::{
    HttpRequest, HttpResponse, Responder,
    error::{ErrorInternalServerError, ErrorNotFound},
    post, web,
};
use serde_json::{Value, json};
use sqlx::{Pool, Postgres};

use crate::{
    dto::create_comment::Request,
    helper::{get_request_hash, hash_password},
};

#[post("/posts/{id}/comments")]
pub async fn create_comment(
    req: HttpRequest,
    path: web::Path<i32>,
    data: web::Json<Request>,
    pool: web::Data<Pool<Postgres>>,
) -> actix_web::Result<impl Responder> {
    let author_hash = get_request_hash(&req)?;
    let post_id = path.into_inner();
    let Request {
        author: author_name,
        password,
        content,
    } = data.into_inner();

    let mut constraints = HashMap::<&str, Value>::new();
    let author_name = author_name
        .unwrap()
        .map_err(|e| constraints.insert("author", e))
        .ok();
    let password = password
        .unwrap()
        .map_err(|e| constraints.insert("password", e))
        .ok();
    let content = content
        .unwrap()
        .map_err(|e| constraints.insert("content", e))
        .ok();

    if !constraints.is_empty() {
        return Ok(HttpResponse::UnprocessableEntity().json(json!({
            "constraints": constraints
        })));
    }

    let author_name = author_name.unwrap();
    let password = password.unwrap();
    let content = content.unwrap();

    let password_hash = hash_password(&password).map_err(ErrorInternalServerError)?;

    sqlx::query!(
        r#"
        INSERT INTO comments (
            post_id,
            author_name,
            author_hash,
            password_hash,
            content
        )
        VALUES ($1, $2, $3, $4, $5)
        "#,
        post_id,
        author_name,
        author_hash,
        password_hash,
        content
    )
    .execute(pool.get_ref())
    .await
    .map_err(|err| {
        if err
            .as_database_error()
            .is_some_and(|err| err.constraint() == Some("comments_post_id_fkey"))
        {
            ErrorNotFound("Post not found")
        } else {
            ErrorInternalServerError(err)
        }
    })?;

    Ok(HttpResponse::Created().finish())
}
