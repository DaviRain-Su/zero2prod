//! tests/health_check.rs
use std::net::TcpListener;

// `tokio::test` is the testing equivalent of `tokio::main`.
// It also spares you from having to specify the `#[test]` attribute.
//
// You can inspect what code gets generated using
// `cargo expand --test health_check` (<- name of the test file)
#[tokio::test]
async fn health_check_works() {
    // Arrange
    let addr = spawn_app();
    println!("addr : {}", addr);
    // We need to bring in `reqwest`
    // to perform HTTP requests against our application.
    let client = reqwest::Client::new();
    // Act
    let response = client
        .get(&format!("{}/health_check", &addr))
        .send()
        .await
        .expect("Failed to execute request.");
    // Assert
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

// Launch our application in the background ~somehow~
fn spawn_app() -> String {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
    // We retrieve the port assigned to us by the OS
    let port = listener.local_addr().unwrap().port();
    tracing::info!("port: {:?}", port);

    let server = zero2prod::run(listener).expect("faild bind address");

    tokio::spawn(server);

    format!("http://127.0.0.1:{}", port)
}
