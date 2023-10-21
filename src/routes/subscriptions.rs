use actix_web::{HttpResponse, web};
use sqlx::PgPool;
use uuid::Uuid;
use sqlx::types::chrono::Utc;
use tracing::span::Entered;
use tracing::Instrument;

#[derive(serde::Deserialize)]
pub struct FormData {
    email: String,
    name: String,
}

// you can test this with:
// curl -X POST localhost:8000/subscriptions    -H "Content-Type: application/x-www-form-urlencoded"  --data "name=noam&email=g@g.com"

// check the tracing docs: https://docs.rs/tracing/latest/tracing/

pub(crate) async fn subscribe(form: web::Form<FormData>,
                              pool: web::Data<PgPool>, ) -> HttpResponse {
    /* 3.9.3
    web::Data, when a new request comes in, computes the TypeId of the type you specified in the signature (in
our case PgConnection) and checks if there is a record corresponding to it in the type-map. If there is one, it
casts the retrieved Any value to the type you specified (TypeId is unique, nothing to worry about) and passes
it to your handler.
It is an interesting technique to perform what in other language ecosystems might be referred to as depend-
ency injection.
     */

    let request_id = Uuid::new_v4();
    let request_span = tracing::info_span!(
        "Adding a new subscriber.",
        %request_id,
        subscriber_email = %form.email,
        subscriber_name = %form.name
        );
    // Using `enter` in an async function is a recipe for disaster!
    // Bear with me for now, but don't do this at home.
    // See the following section on `Instrumenting Futures`
    let _request_span_guard : Entered= request_span.enter();

    // We do not call `.enter` on query_span!
    // `.instrument` takes care of it at the right moments
    // in the query future lifetime
    let query_span = tracing::info_span!("Saving new subscriber details in the database");
    //tracing::info!("request_id {} - Saving new subscriber details in the database",request_id);

    let result = sqlx::query!(
        r#"
        INSERT INTO subscriptions (id, email, name, subscribed_at)
        VALUES ($1, $2, $3, $4)
        "#,
        Uuid::new_v4(),
        form.email,
        form.name,
        Utc::now()
    )
    // We use `get_ref` to get an immutable reference to the `PgConnection`
    // wrapped by `web::Data`.
    .execute(pool.get_ref())
    .instrument(query_span)
    .await;

    match result{
        Ok(_) => {
            HttpResponse::Ok().finish()
        }
        Err(e) => {
            tracing::error!("request_id {} Failed to execute query: {:?}", request_id, e);
            HttpResponse::InternalServerError().finish()
        }
    }
}
