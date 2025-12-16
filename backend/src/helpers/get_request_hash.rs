use actix_web::HttpRequest;

use crate::helpers::get_x_real_ip;

pub fn get_request_hash(req: &HttpRequest) -> actix_web::Result<String> {
    let ip = get_x_real_ip(req)?;
    Ok(blake3::hash(ip).to_hex().to_string())
}
