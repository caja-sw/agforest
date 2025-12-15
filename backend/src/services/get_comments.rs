use actix_web::{
    HttpResponse, Responder,
    error::{ErrorInternalServerError, ErrorNotFound},
    get, web,
};
use diesel::{dsl::exists, prelude::*, select};

use crate::{
    dao::GetCommentsComment,
    dto::GetCommentsResponse,
    helpers::{DBPool, connect},
};

#[get("/posts/{id}/comments")]
pub async fn get_comments(
    path: web::Path<i32>,
    pool: web::Data<DBPool>,
) -> actix_web::Result<impl Responder> {
    let post_id = path.into_inner();
    let mut connection = connect(pool)?;

    let data = web::block(move || {
        connection.transaction::<Option<Vec<GetCommentsComment>>, diesel::result::Error, _>(
            |connection| {
                use crate::schema::comments::dsl::{self as comment, comments};
                use crate::schema::posts::dsl::{self as post, posts};

                let post_exists: bool =
                    select(exists(posts.filter(post::id.eq(post_id)))).get_result(connection)?;

                if !post_exists {
                    return Ok(None);
                }

                let comments_ = comments
                    .filter(comment::post_id.eq(post_id))
                    .order(comment::created_at.asc())
                    .select((
                        comment::id,
                        comment::author_name,
                        comment::author_hash,
                        comment::content,
                        comment::created_at,
                    ))
                    .load::<GetCommentsComment>(connection)?;

                Ok(Some(comments_))
            },
        )
    })
    .await?
    .map_err(ErrorInternalServerError)?
    .ok_or_else(|| ErrorNotFound("Post not found"))?;

    Ok(HttpResponse::Ok().json(GetCommentsResponse::from(data)))
}
