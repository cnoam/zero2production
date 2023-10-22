use actix_web::{HttpResponse, web};
use sqlx::PgPool;
use uuid::Uuid;
use sqlx::types::chrono::Utc;

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
        request_id = % Uuid::new_v4(),
        subscriber_email = % form.email, subscriber_name = % form.name
    )
)]
pub(crate) async fn subscribe(form: web::Form<FormData>,
                              pool: web::Data<PgPool>, ) -> HttpResponse {

    match insert_subscriber(&pool, &form).await
    {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::InternalServerError().finish()
    }
}


#[tracing::instrument(
    name = "Saving new subscriber details in the database",
    skip(form, pool)
)]
pub async fn insert_subscriber(
    pool: &PgPool,
    form: &FormData,
) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
        INSERT INTO subscriptions (id, email, name, subscribed_at)
        VALUES ($1, $2, $3, $4)
        "#,
        Uuid::new_v4(),
        form.email,
        form.name,
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
