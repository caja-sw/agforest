use std::{
    future::{Ready, ready},
    ops::Deref,
};

use actix_web::{FromRequest, HttpRequest, dev::Payload, error::ErrorBadRequest};

pub struct Password {
    value: Vec<u8>,
}

impl Deref for Password {
    type Target = [u8];

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl FromRequest for Password {
    type Error = actix_web::Error;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        const HEADER_NAME: &str = "Password";
        let res = req
            .headers()
            .get(HEADER_NAME)
            .ok_or_else(|| ErrorBadRequest(format!("Missing required header: {HEADER_NAME}")))
            .map(|header_value| Self {
                value: header_value.as_bytes().to_vec(),
            });
        ready(res)
    }
}
