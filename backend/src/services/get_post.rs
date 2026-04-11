use actix_web::{
    HttpResponse, Responder,
    error::{ErrorInternalServerError, ErrorNotFound},
    get, web,
};
use serde_json::json;
use sqlx::{Pool, Postgres};
use tokio::try_join;

#[get("/posts/{id}")]
pub async fn get_post(
    path: web::Path<i32>,
    pool: web::Data<Pool<Postgres>>,
) -> actix_web::Result<impl Responder> {
    let post_id = path.into_inner();

    let post = sqlx::query!(
        r#"
        SELECT 
            p.id,
            p.author_name,
            p.author_hash,
            p.category_id,
            c.name AS category_name,
            c.readonly AS category_readonly,
            p.title,
            p.content,
            p.view_count,
            p.like_count,
            p.created_at
        FROM posts p
        INNER JOIN categories c ON p.category_id = c.id
        WHERE p.id = $1 AND p.deleted_at IS NULL
        "#,
        post_id
    )
    .fetch_optional(&**pool);

    let comments = sqlx::query!(
        r#"
        SELECT
            id,
            author_name,
            author_hash,
            content,
            created_at
        FROM comments
        WHERE post_id = $1 AND deleted_at IS NULL
        ORDER BY created_at ASC
        "#,
        post_id
    )
    .fetch_all(&**pool);

    let (post, comments) = try_join!(post, comments).map_err(ErrorInternalServerError)?;
    let post = post.ok_or(ErrorNotFound("Post not found"))?;

    Ok(HttpResponse::Ok().json(json!({
        "id": post.id,
        "category": {
            "id": post.category_id,
            "name": post.category_name,
            "readonly": post.category_readonly,
        },
        "author": {
            "name": post.author_name,
            "hash": post.author_hash
        },
        "title": post.title,
        "content": post.content,
        "viewCount": post.view_count,
        "likeCount": post.like_count,
        "createdAt": post.created_at,
        "comments": comments.into_iter().map(|comment| {
            json!({
                "id": comment.id,
                "author": {
                    "name": comment.author_name,
                    "hash": comment.author_hash
                },
                "content": comment.content,
                "createdAt": comment.created_at,
            })
        }).collect::<Vec<_>>()
    })))
}
