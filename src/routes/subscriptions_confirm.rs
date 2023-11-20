//! src/routes/subscriptions_confirm.rs

use actix_web::{HttpResponse, web};
use sqlx::PgPool;
use uuid::Uuid;

/// The Parameters struct defines all the query parameters that we expect to see in the incoming request.

#[derive(serde::Deserialize)]
pub struct Parameters {
    subscription_token: String
}

#[tracing::instrument(
name = "Confirm a pending subscriber",
skip(parameters)
)]
// It is enough to add a function parameter of type web::Query<Parameter> to confirm() to instruct actix-web
// to only call the handler if the extraction was successful. If the extraction failed a 400 Bad Request is auto-
// matically returned to the caller.
pub async fn confirm(parameters: web::Query<Parameters>,
                     pool: web::Data<PgPool>,
) -> HttpResponse {
    let id = match get_subscriber_id_from_token(
        &pool,
        &parameters.subscription_token,
    ).await {
        Ok(id) => id,
        //noam: if the ID cannot be found, maybe the token is corrupt?
        // this is not an internal error.
        Err(_) => return HttpResponse::InternalServerError().finish(),
    };
    match id {
        // Non-existing token!
        None => HttpResponse::Unauthorized().finish(),
        Some(subscriber_id) => {
            if confirm_subscriber(&pool, subscriber_id).await.is_err() {
                return HttpResponse::InternalServerError().finish();
            }
            HttpResponse::Ok().finish()
        }
    }
}


#[tracing::instrument(
name = "Mark subscriber as confirmed",
skip(subscriber_id, pool)
)]
pub async fn confirm_subscriber(
    pool: &PgPool,
    subscriber_id: Uuid,
) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"UPDATE subscriptions SET status = 'confirmed' WHERE id = $1"#,subscriber_id)
        .execute(pool)
        .await
        .map_err(|e| {
            tracing::error!("Failed to execute query: {:?}", e);
            e
        })?;
    Ok(())
}

#[tracing::instrument(
name = "Get subscriber_id from token",
skip(subscription_token, pool)
)]
pub async fn get_subscriber_id_from_token(
    pool: &PgPool,
    subscription_token: &str,
) -> Result<Option<Uuid>, sqlx::Error> {
    let result = sqlx::query!(
        "SELECT subscriber_id FROM subscription_tokens \
            WHERE subscription_token = $1",
        subscription_token,
    )
        .fetch_optional(pool)
        .await
        .map_err(|e| {
            tracing::error!("Failed to execute query: {:?}", e);
            e
        })?;
    Ok(result.map(|r| r.subscriber_id))
}