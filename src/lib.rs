//! lib.rs
use actix_web::{App, HttpResponse, HttpServer, web};
use actix_web::dev::Server;

async fn health_check() -> HttpResponse {
    HttpResponse::Ok().finish()
}


pub  fn run() -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| {
        App::new()
            .route("/health_check", web::get().to(health_check))
    })
        .bind("127.0.0.1:8000")?
        .run();
        // do not wait since it will block the integration test runs.
        // .await
    Ok(server)
}
//
// pub async fn run() -> Result<(), std::io::Error> {
//     HttpServer::new(|| {
//         App::new()
//             .route("/health_check", web::get().to(health_check))
//     })
//         .bind("127.0.0.1:8000")?
//         .run()
//         .await
// }