use actix_web::{HttpRequest, error::ErrorBadRequest};

pub fn get_password(req: &HttpRequest) -> actix_web::Result<&[u8]> {
    Ok(req
        .headers()
        .get("Password")
        .ok_or_else(|| ErrorBadRequest("Missing required header: Password"))?
        .to_str()
        .map_err(|_| ErrorBadRequest("Invalid header format: Password"))?
        .as_bytes())
}
