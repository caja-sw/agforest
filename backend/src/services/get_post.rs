use actix_web::{
    HttpResponse, Responder,
    error::{ErrorInternalServerError, ErrorNotFound},
    get, web,
};
use diesel::prelude::*;

use crate::{
    dao::GetPostPost,
    dto::GetPostResponse,
    helpers::{DBPool, connect},
};

#[get("/posts/{id}")]
pub async fn get_post(
    path: web::Path<i32>,
    pool: web::Data<DBPool>,
) -> actix_web::Result<impl Responder> {
    let post_id = path.into_inner();
    let mut connection = connect(pool)?;

    let data = web::block(move || {
        use crate::schema::categories::dsl::{self as category, categories};
        use crate::schema::posts::dsl::{self as post, posts};

        posts
            .inner_join(categories.on(category::id.eq(post::category_id)))
            .filter(post::id.eq(post_id))
            .select((
                post::id,
                post::author_name,
                post::author_hash,
                post::category_id,
                category::name,
                post::title,
                post::content,
                post::created_at,
            ))
            .first::<GetPostPost>(&mut connection)
            .optional()
    })
    .await?
    .map_err(ErrorInternalServerError)?
    .ok_or_else(|| ErrorNotFound("Post not found"))?;

    Ok(HttpResponse::Ok().json(GetPostResponse::from(data)))
}
