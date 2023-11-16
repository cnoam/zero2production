//! startup.rs
use actix_web::{App, HttpServer, web};
use actix_web::dev::Server;
use std::net::TcpListener;
use sqlx::PgPool;
use crate::routes::{health_check::health_check, subscriptions::subscribe};
use tracing_actix_web::TracingLogger;
use crate::email_client:: EmailClient;

pub  fn run(listener: TcpListener, db_pool: PgPool,email_client: EmailClient,) -> Result<Server, std::io::Error> {
    // Wrap the connection in a smart pointer
    let db_pool = web::Data::new(db_pool);
    // Capture `connection` from the surrounding environment ---> add "move"
    let server = HttpServer::new(move || {
        App::new()
            // Middlewares are added using the `wrap` method on `App`
            .wrap(TracingLogger::default())
            .route("/health_check", web::get().to(health_check))
            .route("/subscriptions", web::post().to(subscribe))
            // Register the connection as part of the application state
            .app_data(db_pool.clone()) // this will be used in src/routes/subscriptions handler
            .app_data(email_client.clone())
    })
        .listen(listener)?
        .run();
    Ok(server)
}