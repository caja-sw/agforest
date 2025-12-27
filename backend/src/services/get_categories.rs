use actix_web::{HttpResponse, Responder, error::ErrorInternalServerError, get, web};
use sqlx::{Pool, Postgres};

use crate::{dto::get_categories::Response, entity::get_categories::CategoryEntity};

#[get("/categories")]
pub async fn get_categories(pool: web::Data<Pool<Postgres>>) -> actix_web::Result<impl Responder> {
    let categories = sqlx::query_as!(
        CategoryEntity,
        r#"
        SELECT
            id,
            name,
            readonly
        FROM categories
        "#
    )
    .fetch_all(pool.get_ref())
    .await
    .map_err(ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(Response::from(categories)))
}
