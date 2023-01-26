use std::net::TcpListener;

fn spawn_app() -> String {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind to random port");

    let port = listener.local_addr().unwrap().port();

    let server = zero2prod::run(listener).expect("Failed to bind address");
    let _ = tokio::spawn(server);

    format!("http://localhost:{}", port)
}

#[tokio::test]
async fn health_check_works() {
    let address = spawn_app();

    let client = reqwest::Client::new();

    let response = client
        .get(&format!("{}/health_check", &address))
        .send()
        .await
        .expect("Failed to execute request");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

#[tokio::test]
async fn subscribe_returns_a_200_for_valid_form_data() {
    //Arrange
    let app_address = spawn_app();
    let client = reqwest::Client::new();

    //Act
    let body = "email=ursula_le_guin%40gmail.com&name=le520guin";
    let response = client
        .post(&format!("{}/subscriptions", &app_address))
        .header("Content-Type", "applications/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to execute request");

    //Assert
    //return 400 ATM
    assert_eq!(200, response.status().as_u16());
}

#[tokio::test]
async fn subscribe_returns_a_400_for_when_data_is_missing() {
    //Arrange
    let address = spawn_app();
    let client = reqwest::Client::new();
    let test_cases = vec![
        ("name=le%20quin", "missing the email"),
        ("email=ursula_la_qui%40gmail.com", "missing the name"),
        ("", "missing both email and name"),
    ];

    // Act
    for (invalid_body, error_message) in test_cases {
        let response = client
            .post(&format!("{}/subscriptions", &address))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(invalid_body)
            .send()
            .await
            .expect("Post request not executed");

        //Assert
        assert_eq!(
            400,
            response.status().as_u16(),
            // Custom Error message
            "The API did not fail with a 400 bad request when the payload was {}",
            error_message
        );
    }
}
