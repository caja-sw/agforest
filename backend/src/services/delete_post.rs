use actix_web::{
    HttpRequest, HttpResponse, Responder, delete,
    error::{ErrorForbidden, ErrorInternalServerError, ErrorNotFound},
    web,
};
use argon2::{Argon2, PasswordHash, PasswordVerifier};
use diesel::{dsl::delete, prelude::*};

use crate::{
    dao::DeletePostPost,
    helpers::{DBPool, connect, get_password},
};

#[delete("/posts/{id}")]
pub async fn delete_post(
    req: HttpRequest,
    path: web::Path<i32>,
    pool: web::Data<DBPool>,
) -> actix_web::Result<impl Responder> {
    let password = get_password(&req)?.to_owned();
    let post_id = path.into_inner();
    let mut connection = connect(pool)?;

    enum Error {
        Forbidden,
        NotFound,
    }

    web::block(move || {
        connection.transaction::<_, diesel::result::Error, _>(|connection| {
            use crate::schema::posts::dsl::{self as post, posts};

            let post = posts
                .filter(post::id.eq(post_id))
                .select((post::password_hash,))
                .first::<DeletePostPost>(connection)
                .optional()?;

            if post.is_none() {
                return Ok(Err(Error::NotFound));
            }

            let post = post.unwrap();

            let matches = {
                let hash = PasswordHash::new(&post.password_hash)
                    .map_err(|_| diesel::result::Error::RollbackTransaction)?;
                let argon2 = Argon2::default();
                argon2.verify_password(&password, &hash).is_ok()
            };

            if !matches {
                return Ok(Err(Error::Forbidden));
            }

            delete(posts.filter(post::id.eq(post_id))).execute(connection)?;

            Ok(Ok(()))
        })
    })
    .await?
    .map_err(ErrorInternalServerError)?
    .map_err(|e| match e {
        Error::Forbidden => ErrorForbidden("Incorrect password"),
        Error::NotFound => ErrorNotFound("Post not found"),
    })?;

    Ok(HttpResponse::NoContent())
}
