use std::collections::HashMap;

use actix_web::{
    HttpRequest, HttpResponse, Responder,
    error::{ErrorInternalServerError, ErrorNotFound},
    post, web,
};
use serde_json::{Value, json};
use sqlx::{Pool, Postgres};

use crate::{
    dto::create_post::{Request, Response},
    entity::create_post::PostEntity,
    helper::{get_request_hash, hash_password},
};

#[post("/categories/{id}/posts")]
pub async fn create_post(
    req: HttpRequest,
    path: web::Path<i32>,
    data: web::Json<Request>,
    pool: web::Data<Pool<Postgres>>,
) -> actix_web::Result<impl Responder> {
    let author_hash = get_request_hash(&req)?;
    let category_id = path.into_inner();
    let Request {
        author: author_name,
        password,
        title,
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
    let title = title
        .unwrap()
        .map_err(|e| constraints.insert("title", e))
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
    let title = title.unwrap();
    let content = content.unwrap();

    let password_hash = hash_password(&password).map_err(ErrorInternalServerError)?;

    let post = sqlx::query_as!(
        PostEntity,
        r#"
        INSERT INTO posts (
            category_id,
            author_name,
            author_hash,
            password_hash,
            title,
            content
        )
        VALUES ($1, $2, $3, $4, $5, $6)
        RETURNING id
        "#,
        category_id,
        author_name,
        author_hash,
        password_hash,
        title,
        content
    )
    .fetch_one(pool.get_ref())
    .await
    .map_err(|err| {
        if err
            .as_database_error()
            .is_some_and(|err| err.constraint() == Some("posts_category_id_fkey"))
        {
            ErrorNotFound("Category not found")
        } else {
            ErrorInternalServerError(err)
        }
    })?;

    Ok(HttpResponse::Created().json(Response::from(post)))
}
