use actix_web::{HttpResponse, Responder, error::ErrorInternalServerError, get, web};
use serde_json::json;
use sqlx::{Pool, Postgres};

#[get("/writables")]
pub async fn get_writables(pool: web::Data<Pool<Postgres>>) -> actix_web::Result<impl Responder> {
    let categories = sqlx::query!(
        r#"
        SELECT
            id,
            name
        FROM categories
        WHERE readonly = false
        "#
    )
    .fetch_all(&**pool)
    .await
    .map_err(ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(json!({
        "categories": categories.into_iter().map(|category| {
            json!({
                "id": category.id,
                "name": category.name,
            })
        }).collect::<Vec<_>>(),
    })))
}
