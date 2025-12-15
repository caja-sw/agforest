use actix_web::{
    HttpRequest, HttpResponse, Responder, delete,
    error::{ErrorForbidden, ErrorInternalServerError, ErrorNotFound},
    web,
};
use argon2::{Argon2, PasswordHash, PasswordVerifier};
use diesel::{dsl::delete, prelude::*};

use crate::{
    dao::DeleteCommentComment,
    helpers::{DBPool, connect, get_password},
};

#[delete("/comments/{id}")]
pub async fn delete_comment(
    req: HttpRequest,
    path: web::Path<i32>,
    pool: web::Data<DBPool>,
) -> actix_web::Result<impl Responder> {
    let password = get_password(&req)?.to_owned();
    let comment_id = path.into_inner();
    let mut connection = connect(pool)?;

    enum Error {
        Forbidden,
        NotFound,
    }

    web::block(move || {
        connection.transaction::<_, diesel::result::Error, _>(|connection| {
            use crate::schema::comments::dsl::{self as comment, comments};

            let comment = comments
                .filter(comment::id.eq(comment_id))
                .select((comment::password_hash,))
                .first::<DeleteCommentComment>(connection)
                .optional()?;

            if comment.is_none() {
                return Ok(Err(Error::NotFound));
            }

            let comment = comment.unwrap();

            let matches = {
                let hash = PasswordHash::new(&comment.password_hash)
                    .map_err(|_| diesel::result::Error::RollbackTransaction)?;
                let argon2 = Argon2::default();
                argon2.verify_password(&password, &hash).is_ok()
            };

            if !matches {
                return Ok(Err(Error::Forbidden));
            }

            delete(comments.filter(comment::id.eq(comment_id))).execute(connection)?;

            Ok(Ok(()))
        })
    })
    .await?
    .map_err(ErrorInternalServerError)?
    .map_err(|e| match e {
        Error::Forbidden => ErrorForbidden("Incorrect password"),
        Error::NotFound => ErrorNotFound("Comment not found"),
    })?;

    Ok(HttpResponse::NoContent())
}
