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
    dto::{CreateCommentRequest, CreateCommentResponse},
    helpers::{DBPool, connect, get_request_hash, hash_password},
};

#[post("/posts/{id}/comments")]
pub async fn create_comment(
    req: HttpRequest,
    path: web::Path<i32>,
    data: web::Json<CreateCommentRequest>,
    pool: web::Data<DBPool>,
) -> actix_web::Result<impl Responder> {
    let author_hash = get_request_hash(&req)?;
    let post_id = path.into_inner();
    let CreateCommentRequest {
        author: author_name,
        password,
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
    let content = content
        .unwrap()
        .map_err(|e| message_map.insert("content", e))
        .ok();

    if !message_map.is_empty() {
        return Ok(HttpResponse::UnprocessableEntity().json(message_map));
    }

    let author_name = author_name.unwrap();
    let password = password.unwrap();
    let content = content.unwrap();

    let mut connection = connect(pool)?;

    let password_hash = hash_password(&password).map_err(ErrorInternalServerError)?;

    let id = web::block(move || {
        connection.transaction::<Option<i32>, diesel::result::Error, _>(|connection| {
            use crate::schema::comments::dsl::{self as comment, comments};
            use crate::schema::posts::dsl::{self as post, posts};

            let post_exists: bool =
                select(exists(posts.filter(post::id.eq(post_id)))).get_result(connection)?;

            if !post_exists {
                return Ok(None);
            }

            let id: i32 = insert_into(comments)
                .values((
                    comment::author_name.eq::<&str>(&author_name),
                    comment::author_hash.eq(&author_hash),
                    comment::password_hash.eq(&password_hash),
                    comment::post_id.eq(post_id),
                    comment::content.eq::<&str>(&content),
                ))
                .returning(comment::id)
                .get_result(connection)?;

            Ok(Some(id))
        })
    })
    .await?
    .map_err(ErrorInternalServerError)?
    .ok_or_else(|| ErrorNotFound("Post not found"))?;

    Ok(HttpResponse::Created().json(CreateCommentResponse { id }))
}
