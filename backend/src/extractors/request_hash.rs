use std::{
    future::{Ready, ready},
    ops::Deref,
};

use actix_web::{FromRequest, HttpRequest, dev::Payload, error::ErrorBadRequest};

pub struct RequestHash {
    value: String,
}

impl Deref for RequestHash {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl FromRequest for RequestHash {
    type Error = actix_web::Error;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        const HEADER_NAME: &str = "X-Real-IP";
        let res = req
            .headers()
            .get(HEADER_NAME)
            .ok_or_else(|| ErrorBadRequest(format!("Missing required header: {HEADER_NAME}")))
            .map(|header_value| Self {
                value: blake3::hash(header_value.as_bytes()).to_hex().to_string(),
            });

        ready(res)
    }
}
