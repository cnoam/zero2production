//! main.rs
//use std::net::TcpListener;
use zero2prod::run;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    // let listerner =
    //     TcpListener::bind("127.0.0.1:8000")
    //         .expect("Failed to bind to 8000");
    // run(listerner)?.await
    run().await
}

