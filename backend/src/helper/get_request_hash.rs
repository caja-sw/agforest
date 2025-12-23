use actix_web::{HttpRequest, error::ErrorBadRequest};

pub fn get_request_hash(req: &HttpRequest) -> actix_web::Result<String> {
    const HEADER_X_REAL_IP: &str = "X-Real-IP";
    let ip = req
        .headers()
        .get(HEADER_X_REAL_IP)
        .ok_or_else(|| ErrorBadRequest(format!("Missing required header: {}", HEADER_X_REAL_IP)))?
        .to_str()
        .map_err(|_| ErrorBadRequest(format!("Invalid header format: {}", HEADER_X_REAL_IP)))?
        .as_bytes();
    Ok(blake3::hash(ip).to_hex().to_string())
}
