use std::collections::HashMap;

use actix_web::{
    HttpRequest, HttpResponse, Responder,
    error::{ErrorInternalServerError, ErrorNotFound},
    post, web,
};
use diesel::{
    dsl::{exists, insert_into},
    prelude::*,
    select,
};

use crate::{
    dto::{CreatePostRequest, CreatePostResponse},
    helpers::{DBPool, connect, get_request_hash, hash_password},
};

#[post("/categories/{id}/posts")]
pub async fn create_post(
    req: HttpRequest,
    path: web::Path<i32>,
    data: web::Json<CreatePostRequest>,
    pool: web::Data<DBPool>,
) -> actix_web::Result<impl Responder> {
    let author_hash = get_request_hash(&req)?;
    let category_id = path.into_inner();
    let CreatePostRequest {
        author: author_name,
        password,
        title,
        content,
    } = data.into_inner();

    let mut message_map = HashMap::<&str, String>::new();
    let author_name = author_name
        .unwrap()
        .map_err(|e| message_map.insert("author", e))
        .ok();
    let password = password
        .unwrap()
        .map_err(|e| message_map.insert("password", e))
        .ok();
    let title = title
        .unwrap()
        .map_err(|e| message_map.insert("title", e))
        .ok();
    let content = content
        .unwrap()
        .map_err(|e| message_map.insert("content", e))
        .ok();

    if !message_map.is_empty() {
        return Ok(HttpResponse::UnprocessableEntity().json(message_map));
    }

    let author_name = author_name.unwrap();
    let password = password.unwrap();
    let title = title.unwrap();
    let content = content.unwrap();

    let mut connection = connect(pool)?;

    let password_hash = hash_password(&password).map_err(ErrorInternalServerError)?;

    let id = web::block(move || {
        connection.transaction::<Option<i32>, diesel::result::Error, _>(|connection| {
            use crate::schema::categories::dsl::{self as category, categories};
            use crate::schema::posts::dsl::{self as post, posts};

            let category_exists: bool =
                select(exists(categories.filter(category::id.eq(category_id))))
                    .get_result(connection)?;

            if !category_exists {
                return Ok(None);
            }

            let id: i32 = insert_into(posts)
                .values((
                    post::author_name.eq::<&str>(&author_name),
                    post::author_hash.eq(&author_hash),
                    post::password_hash.eq(&password_hash),
                    post::category_id.eq(category_id),
                    post::title.eq::<&str>(&title),
                    post::content.eq::<&str>(&content),
                ))
                .returning(post::id)
                .get_result(connection)?;

            Ok(Some(id))
        })
    })
    .await?
    .map_err(ErrorInternalServerError)?
    .ok_or_else(|| ErrorNotFound("Category not found"))?;

    Ok(HttpResponse::Created().json(CreatePostResponse { id }))
}
