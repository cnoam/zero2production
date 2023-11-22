use actix_web::{HttpResponse, web};
use rand::{Rng, thread_rng};
use rand::distributions::Alphanumeric;
use sqlx::{Executor, Postgres, Transaction};
use sqlx::PgPool;
use sqlx::types::chrono::Utc;
use uuid::Uuid;
use crate::domain::*;
use crate::email_client::EmailClient;
use crate::startup::ApplicationBaseUrl;
use actix_web::ResponseError;

#[derive(serde::Deserialize)]
pub struct FormData {
    email: String,
    name: String,
}


impl TryFrom<FormData> for NewSubscriber {
    type Error = String;
    fn try_from(value: FormData) -> Result<Self, Self::Error> {
        let name = SubscriberName::parse(value.name)?;
        let email = SubscriberEmail::parse(value.email)?;
        Ok(Self { email, name })
    }
}

// you can test this with:
// curl -X POST localhost:8000/subscriptions    -H "Content-Type: application/x-www-form-urlencoded"  --data "name=noam&email=g@g.com"

// check the tracing docs: https://docs.rs/tracing/latest/tracing/
#[tracing::instrument(
name = "Adding a new subscriber",
skip(form, pool, email_client, base_url),
fields(
subscriber_email = % form.email,
subscriber_name = % form.name
)
)]
pub(crate) async fn subscribe(form: web::Form<FormData>,
                              pool: web::Data<PgPool>,
                              email_client: web::Data<EmailClient>,
                              base_url: web::Data<ApplicationBaseUrl>) -> Result<HttpResponse, actix_web::Error>{
    let new_subscriber = match form.0.try_into() {
        // noam: it is a bit strange that returning BadRequest error is wrapped by Ok() and not
        // by Err(), but this is the correct behavior
        Ok(form) => form,
        Err(_) => return Ok(HttpResponse::BadRequest().finish()),
    };
    let mut transaction = match pool.begin().await {
        Ok(transaction) => transaction,
        Err(_) => return Ok(HttpResponse::InternalServerError().finish()),
    };
    let subscriber_id = match insert_subscriber(&mut transaction, &new_subscriber).await {
        Ok(subscriber_id) => subscriber_id,
        Err(_) => return Ok(HttpResponse::InternalServerError().finish()),
    };
    let subscription_token = generate_subscription_token();

    // The `?` operator transparently invokes the `Into` trait
    // on our behalf - we don't need an explicit `map_err` anymore.
    store_token(&mut transaction, subscriber_id, &subscription_token)
        .await?;

    if transaction.commit().await.is_err() {
        return Ok(HttpResponse::InternalServerError().finish());
    }
    if send_confirmation_email(&email_client, new_subscriber, &base_url.0, &subscription_token)
        .await
        .is_err() {
        //TODO Log("");
        return Ok(HttpResponse::InternalServerError().finish());
    }
    Ok(HttpResponse::Ok().finish())
}

#[tracing::instrument(
name = "Send a confirmation email to a new subscriber",
skip(email_client, new_subscriber, base_url, subscription_token)
)]
pub async fn send_confirmation_email(email_client: &EmailClient,
                                     new_subscriber: NewSubscriber,
                                     base_url: &str,
                                     subscription_token: &str,
) -> Result<(), reqwest::Error> {
    let confirmation_link = format!(
        "{}/subscriptions/confirm?subscription_token={}",
        base_url,
        subscription_token
    );
    let plain_body = format!(
        "Welcome to our newsletter!\nVisit {} to confirm your subscription.",
        confirmation_link
    );
    let html_body = format!("Welcome to our newsletter!<br />\
Click <a href=\"{}\">here</a> to confirm your subscription.",
                            confirmation_link);

    email_client.send_email(new_subscriber.email, "Welcome!", &html_body, &plain_body)
        .await
}


#[tracing::instrument(
name = "Saving new subscriber details in the database",
skip(transaction)
)]
async fn insert_subscriber(
    transaction: &mut Transaction<'_, Postgres>,
    new_subscriber: &NewSubscriber,
) -> Result<Uuid, sqlx::Error> {
    let subscriber_id = Uuid::new_v4();
    let query = sqlx::query!(
        r#"
        INSERT INTO subscriptions (id, email, name, subscribed_at, status)
        VALUES ($1, $2, $3, $4, 'pending_confirmation')"#,
        subscriber_id,
        new_subscriber.email.as_ref(),
        new_subscriber.name.as_ref(),
        Utc::now()
    );
    transaction.execute(query)
        .await
        .map_err(|e| {
            tracing::error!("Failed to execute query: {:?}", e);
            e
        })?;
    Ok(subscriber_id)
}

/// Generate a random 25-characters-long case-sensitive subscription token.
fn generate_subscription_token() -> String {
    let mut rng = thread_rng();
    std::iter::repeat_with(|| rng.sample(Alphanumeric))
        .map(char::from)
        .take(25)
        .collect()
}

#[tracing::instrument(
name = "Store subscription token in the database",
skip(subscription_token, transaction)
)]
pub async fn store_token(transaction: &mut Transaction<'_, Postgres>, subscriber_id: Uuid, subscription_token: &str)
                         -> Result<(),StoreTokenError> {
    let query = sqlx::query!(
        r#"INSERT INTO subscription_tokens (subscription_token, subscriber_id)
        VALUES ($1, $2)"#,
        subscription_token,
        subscriber_id
    );
    transaction.execute(query)
        .await
        .map_err(|e| {
            tracing::error!("Failed to execute query: {:?}", e);
            StoreTokenError(e)
        })?;
    Ok(())
}

// A new error type, wrapping a sqlx::Error
// We derive `Debug`, easy and painless.
#[derive(Debug)]
pub struct StoreTokenError(sqlx::Error);

impl ResponseError for StoreTokenError {}

impl std::fmt::Display for StoreTokenError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f,"A database error was encountered while \
trying to store a subscription token.")
    }
}