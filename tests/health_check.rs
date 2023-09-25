//! tests/health_check.rs
// `tokio::test` is the testing equivalent of `tokio::main`.
// It also spares you from having to specify the `#[test]` attribute.
//
// You can inspect what code gets generated using
// `cargo expand --test health_check` (<- name of the test file)
#[tokio::test]
async fn health_check_works() {
    // Arrange
    spawn_app();
    //let address: String = spawn_app();
    // We need to bring in `reqwest`
    // to perform HTTP requests against our application.
    let client = reqwest::Client::new();
    let address = "http://localhost:8000";
    // Act
    let response = client
        .get(&format!("{}/health_check", &address))
        .send()
        .await
        .expect("Failed to execute request.");

// Assert
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

// Launch our application in the background
// fn spawn_app() -> String{
//     let listener = std::net::TcpListener::bind("127.0.0.1:0")
//         .expect("Failed to bind random port");
//     let port = listener.local_addr().unwrap().port();
//    let server = zero2prod::run(listener).expect("Failed to bind address");
//    let _ = tokio::spawn(server);
//     format!("http://127.0.0.1:{}", port)
// }

fn spawn_app() {
    let server = zero2prod::run().expect("Failed to bind address");
    // Launch the server as a background task
    // tokio::spawn returns a handle to the spawned future,
    // but we have no use for it here, hence the non-binding let
    let _ = tokio::spawn(server);
}