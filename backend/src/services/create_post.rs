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
    title: String,
    content: String,
}

#[post("/categories/{id}/posts")]
pub async fn create_post(
    path: web::Path<i32>,
    req_hash: extractors::RequestHash,
    data: web::Json<Request>,
    pool: web::Data<Pool<Postgres>>,
) -> actix_web::Result<impl Responder> {
    let category_id = path.into_inner();

    let mut constraints = HashMap::new();
    validate_author("author", &data.author, &mut constraints);
    validate_password("password", &data.password, &mut constraints);
    validate_post_title("title", &data.title, &mut constraints);
    validate_post_content("content", &data.content, &mut constraints);
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

    let post = sqlx::query!(
        r#"
        INSERT INTO posts (
            category_id,
            author_name,
            author_hash,
            password_hash,
            title,
            content
        )
        SELECT $1, $2, $3, $4, $5, $6
        FROM categories
        WHERE id = $1 AND readonly = false
        RETURNING id
        "#,
        category_id,
        &*data.author,
        &*req_hash,
        password_hash,
        &*data.title,
        &*data.content
    )
    .fetch_optional(&**pool)
    .await
    .map_err(ErrorInternalServerError)?
    .ok_or(ErrorForbidden("Category does not exist or is readonly"))?;

    Ok(HttpResponse::Ok().json(json!({
        "id": post.id,
    })))
}
