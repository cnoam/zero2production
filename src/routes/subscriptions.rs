use actix_web::{HttpResponse, web};
use sqlx::PgPool;
use uuid::Uuid;
use sqlx::types::chrono::Utc;

use crate::domain::*;

#[derive(serde::Deserialize)]
pub struct FormData {
    email: String,
    name: String,
}

// you can test this with:
// curl -X POST localhost:8000/subscriptions    -H "Content-Type: application/x-www-form-urlencoded"  --data "name=noam&email=g@g.com"

// check the tracing docs: https://docs.rs/tracing/latest/tracing/
#[tracing::instrument(
name = "Adding a new subscriber",
skip(form, pool),
fields(
subscriber_email = % form.email,
subscriber_name = % form.name
)
)]
pub(crate) async fn subscribe(form: web::Form<FormData>,
                              pool: web::Data<PgPool>, ) -> HttpResponse {
    let name = match SubscriberName::parse(form.0.name){
        Ok(name) => name,
        Err(_) => return HttpResponse::BadRequest().finish(),
    };
      let email = match SubscriberEmail::parse(form.0.email){
        Ok(em) => em,
        Err(_) => return HttpResponse::BadRequest().finish(),
    };
    let subscriber = NewSubscriber {
        email,
        name
    };
    match insert_subscriber(&pool, &subscriber).await {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::InternalServerError().finish()
    }
}


#[tracing::instrument(
name = "Saving new subscriber details in the database",
skip(pool)
)]
pub async fn insert_subscriber(
    pool: &PgPool,
    new_subscriber: &NewSubscriber,
) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
        INSERT INTO subscriptions (id, email, name, subscribed_at)
        VALUES ($1, $2, $3, $4)
        "#,
        Uuid::new_v4(),
        new_subscriber.email.as_ref(),
        new_subscriber.name.as_ref(),
        Utc::now()
    )
        .execute(pool)
        .await
        .map_err(|e| {
            tracing::error!("Failed to execute query: {:?}", e);
            e
        })?;
    Ok(())
}
