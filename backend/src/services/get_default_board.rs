use actix_web::{HttpResponse, Responder, error::ErrorInternalServerError, get, web};
use serde_json::json;
use sqlx::{Pool, Postgres};

#[get("/board/default")]
pub async fn get_default_board(
    pool: web::Data<Pool<Postgres>>,
) -> actix_web::Result<impl Responder> {
    let category = sqlx::query!(
        r#"
        SELECT id
        FROM categories
        LIMIT 1
        "#
    )
    .fetch_one(&**pool)
    .await
    .map_err(ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(json!({
        "id": category.id,
    })))
}
