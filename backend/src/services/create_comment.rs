use std::collections::HashMap;

use actix_web::{
    HttpResponse, Responder,
    error::{ErrorForbidden, ErrorInternalServerError},
    post, web,
};
use argon2::{
    Argon2, PasswordHasher,
    password_hash::{SaltString, rand_core::OsRng},
};
use serde::Deserialize;
use serde_json::json;
use sqlx::{Pool, Postgres};

use crate::{extractors, validators::*};

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct Request {
    author: String,
    password: String,
    content: String,
}

#[post("/posts/{id}/comments")]
pub async fn create_comment(
    path: web::Path<i32>,
    req_hash: extractors::RequestHash,
    data: web::Json<Request>,
    pool: web::Data<Pool<Postgres>>,
) -> actix_web::Result<impl Responder> {
    let post_id = path.into_inner();

    let mut constraints = HashMap::new();
    validate_author("author", &data.author, &mut constraints);
    validate_password("password", &data.password, &mut constraints);
    validate_comment_content("content", &data.content, &mut constraints);
    if !constraints.is_empty() {
        return Ok(HttpResponse::UnprocessableEntity().json(json!({
            "constraints": constraints,
        })));
    }

    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let password_hash = argon2
        .hash_password(data.password.as_bytes(), &salt)
        .map(|hash| hash.to_string())
        .map_err(ErrorInternalServerError)?;

    sqlx::query!(
        r#"
        INSERT INTO comments (
            post_id,
            author_name,
            author_hash,
            password_hash,
            content
        )
        SELECT $1, $2, $3, $4, $5
        FROM posts
        WHERE id = $1
        RETURNING id
        "#,
        post_id,
        &*data.author,
        &*req_hash,
        password_hash,
        &*data.content
    )
    .fetch_optional(&**pool)
    .await
    .map_err(ErrorInternalServerError)?
    .ok_or(ErrorForbidden("Post not found"))?;

    Ok(HttpResponse::Ok().finish())
}
