use actix_web::{HttpRequest, error::ErrorBadRequest};

pub fn get_password(req: &HttpRequest) -> actix_web::Result<&[u8]> {
    const HEADER_PASSWORD: &str = "Password";
    Ok(req
        .headers()
        .get(HEADER_PASSWORD)
        .ok_or_else(|| ErrorBadRequest(format!("Missing required header: {}", HEADER_PASSWORD)))?
        .to_str()
        .map_err(|_| ErrorBadRequest(format!("Invalid header format: {}", HEADER_PASSWORD)))?
        .as_bytes())
}
