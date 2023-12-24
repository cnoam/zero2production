//!src/routes/login/get.rs
use actix_web::{HttpResponse, web};

use actix_web::http::header::ContentType;
use crate::startup::HmacSecret;
use hmac::{Hmac, Mac};
use secrecy::ExposeSecret;

pub async fn login_form(query: Option<web::Query<QueryParams>>,
                        secret: web::Data<HmacSecret>,
) -> HttpResponse {
    let error_html = match query { //noam TODO -- the var should be used
        None => "".into(),
        Some(query) =>
            match query.0.verify(&secret) {
                Ok(error) => {
                    format!("<p><i>{}</i></p>", htmlescape::encode_minimal(&error))
                }
                Err(e) => {
                    tracing::warn!(
                        error.message = %e,
                        error.cause_chain = ?e,
                        "Failed to verify query parameters using the HMAC tag"
                        );
                    "".into()
                }
            }
    };
    HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(include_str!("login.html"))
}

#[derive(serde::Deserialize)]
pub struct QueryParams {
    // page 457
    error: String,
    tag: String,
}

impl QueryParams {
    fn verify(self, secret: &HmacSecret) -> Result<String, anyhow::Error> {
        let tag = hex::decode(self.tag)?;
        let query_string = format!(
            "error={}",
            urlencoding::Encoded::new(&self.error)
        );
        let mut mac = Hmac::<sha2::Sha256>::new_from_slice(
            secret.0.expose_secret().as_bytes()
        ).unwrap();
        mac.update(query_string.as_bytes());
        mac.verify_slice(&tag)?;

        Ok(self.error)
    }
}
