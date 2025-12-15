use actix_web::{HttpResponse, Responder, error::ErrorInternalServerError, get, web};
use diesel::prelude::*;

use crate::{
    dao::GetCategoriesCategory,
    dto::GetCategoriesResponse,
    helpers::{DBPool, connect},
};

#[get("/categories")]
pub async fn get_categories(pool: web::Data<DBPool>) -> actix_web::Result<impl Responder> {
    let mut connection = connect(pool)?;

    let data = web::block(move || {
        use crate::schema::categories::dsl::{self as category, categories};

        categories
            .select((category::id, category::name))
            .load::<GetCategoriesCategory>(&mut connection)
    })
    .await?
    .map_err(ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(GetCategoriesResponse::from(data)))
}
