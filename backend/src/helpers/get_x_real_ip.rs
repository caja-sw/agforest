use actix_web::{HttpRequest, error::ErrorBadRequest};

pub fn get_x_real_ip(req: &HttpRequest) -> actix_web::Result<&[u8]> {
    let key = "X-Real-IP";
    Ok(req
        .headers()
        .get(key)
        .ok_or_else(|| ErrorBadRequest(format!("Missing required header: {}", key)))?
        .to_str()
        .map_err(|_| ErrorBadRequest(format!("Invalid header format: {}", key)))?
        .as_bytes())
}
