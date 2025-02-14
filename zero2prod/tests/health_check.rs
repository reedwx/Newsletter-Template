//use zero2prod::src::main;
use actix_web::rt::spawn;
use std::net::TcpListener;

#[actix_web::test]
async fn health_check_works() {
    let address = spawn_app();
    let client = reqwest::Client::new();
    let _response = client
      .get(&format!("{}/health_check", &address))
      .send()
      .await
      .expect("Failed to execute address");

    //Assert
    assert!(_response.status().is_success());
    assert_eq!(Some(0), _response.content_length());
}

fn spawn_app() -> String {
    let listener = TcpListener::bind("127.0.0.1:0")
      .expect("Failed to bind random port");
    //retrieve port given by OS random binding 0
    let port = listener.local_addr().unwrap().port();
    let server = zero2prod::run(listener).expect("Failed to bind address");
    let _ = spawn(server);
    format!("http://127.0.0.1:{}", port)
}
