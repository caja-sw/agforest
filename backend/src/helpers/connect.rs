use actix_web::{error::ErrorInternalServerError, web};

use crate::helpers::{DBPool, DBPooledConnection};

pub fn connect(pool: web::Data<DBPool>) -> actix_web::Result<DBPooledConnection> {
    pool.get()
        .map_err(|_| ErrorInternalServerError("Connection pool timeout"))
}
