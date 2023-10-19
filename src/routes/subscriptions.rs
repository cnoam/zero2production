use actix_web::{HttpResponse, web};
use sqlx::PgPool;
use uuid::Uuid;
use sqlx::types::chrono::Utc;


#[derive(serde::Deserialize)]
pub struct FormData {
    email: String,
    name: String,
}


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
    .await;

    match result{
        Ok(_) => HttpResponse::Ok().finish(),
        Err(e) => {
            println!("Failed to execute query: {}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}
