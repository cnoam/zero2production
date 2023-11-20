//! src/routes/subscriptions_confirm.rs

use actix_web::{HttpResponse, web};


/// The Parameters struct defines all the query parameters that we expect to see in the incoming request.

// why needed?
#[derive(serde::Deserialize)]
pub struct Parameters {
    subscription_token: String
}

#[tracing::instrument(
name = "Confirm a pending subscriber",
skip(_parameters)
)]
// It is enough to add a function parameter of type web::Query<Parameter> to confirm() to instruct actix-web
// to only call the handler if the extraction was successful. If the extraction failed a 400 Bad Request is auto-
// matically returned to the caller.
pub async fn confirm(_parameters: web::Query<Parameters>) -> HttpResponse {
    HttpResponse::Ok().finish()
}

