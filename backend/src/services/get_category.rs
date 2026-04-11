use actix_web::{
    HttpResponse, Responder,
    error::{ErrorInternalServerError, ErrorNotFound},
    get, web,
};
use serde::Deserialize;
use serde_json::json;
use sqlx::{Pool, Postgres};
use tokio::try_join;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct Query {
    pub limit: i64,
    pub offset: i64,
}

#[get("/categories/{id}")]
pub async fn get_category(
    path: web::Path<i32>,
    query: web::Query<Query>,
    pool: web::Data<Pool<Postgres>>,
) -> actix_web::Result<impl Responder> {
    let category_id = path.into_inner();
    let Query { limit, offset } = *query;

    if !(1..=100).contains(&limit) || offset < 0 {
        return Ok(HttpResponse::BadRequest().finish());
    }

    let category = sqlx::query!(
        r#"
        SELECT
            c.id,
            c.name,
            c.readonly,
            (
                SELECT COUNT(id)
                FROM posts
                WHERE category_id = c.id AND deleted_at IS NULL
            ) AS "total_post_count!"
        FROM categories c
        WHERE c.id = $1
        "#,
        category_id
    )
    .fetch_optional(&**pool);

    let posts = sqlx::query!(
        r#"
        SELECT
            p.id,
            p.author_name,
            p.author_hash,
            p.title,
            p.view_count,
            p.like_count,
            p.created_at,
            (
                SELECT COUNT(id)
                FROM comments
                WHERE post_id = p.id AND deleted_at IS NULL
            ) AS "comment_count!"
        FROM posts p
        WHERE p.category_id = $1 AND p.deleted_at IS NULL
        ORDER BY p.created_at DESC
        LIMIT $2 OFFSET $3
        "#,
        category_id,
        query.limit,
        query.offset
    )
    .fetch_all(&**pool);

    let (category, posts) = try_join!(category, posts).map_err(ErrorInternalServerError)?;
    let category = category.ok_or(ErrorNotFound("Category not found"))?;

    Ok(HttpResponse::Ok().json(json!({
        "id": category.id,
        "name": category.name,
        "readonly": category.readonly,
        "totalPostCount": category.total_post_count,
        "posts": posts.into_iter().map(|post| {
            json!({
                "id": post.id,
                "author": {
                    "name": post.author_name,
                    "hash": post.author_hash,
                },
                "title": post.title,
                "viewCount": post.view_count,
                "likeCount": post.like_count,
                "createdAt": post.created_at,
                "commentCount": post.comment_count,
            })
        }).collect::<Vec<_>>(),
    })))
}
