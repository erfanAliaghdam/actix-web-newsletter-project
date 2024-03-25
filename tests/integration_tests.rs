use std::net::TcpListener;
use reqwest::StatusCode;
use serde_json::Value;
use actix_web_newsletter_project::runner::run;


pub fn spawn_app() -> String {
    let listener = TcpListener::bind("127.0.0.1:0")
        .expect("Failed to bind random port");
    // We retrieve the port assigned to us by the OS
    let port = listener.local_addr().unwrap().port();
    let server = run(listener).expect("Failed to bind address");
    // launch the server at background
    let _ = tokio::spawn(server);
    format!("http://127.0.0.1:{}", port)
}


#[tokio::test]
async fn health_check_works() {
     let address = spawn_app();
     let client = reqwest::Client::new();
     let response = client
         .get(&format!("{}/api/health-check", &address))
         .send()
         .await
         .expect("Failed to execute request.");
     assert_eq!(response.status(), StatusCode::OK);

     let body = response
         .text()
         .await
         .expect("Failed to read response body as text.");

     // Parse the response body as JSON
     let json_body: Value = serde_json::from_str(&body)
         .expect("Failed to parse response body as JSON.");

     assert_eq!(
          json_body,
          serde_json::json!({
            "status": "success",
            "message": "server is working.... ."
        })
     );
}


#[tokio::test]
async fn subscribe_form_returns_200_code_on_valid_data() {
    let address = spawn_app();
    let client = reqwest::Client::new();

    let body = "name=test%20test&email=test%40example.com";
    let response = client
        .post(&format!("{}/api/subscribe", &address))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to execute request.");

    assert_eq!(200, response.status().as_u16());
}

#[tokio::test]
async fn subscribe_form_returns_400_on_data_missing() {
    let app_address = spawn_app();
    let client = reqwest::Client::new();
    let test_cases = vec![
        ("name=John%20Doe", "missing field email"),
        ("email=Johndoe%40example.com", "missing fields name"),
        ("", "missing both name and email")
    ];

    for (invalid_body, error_message) in test_cases {

        let response = client
            .post(&format!("{}/api/subscribe", &app_address))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(invalid_body)
            .send()
            .await
            .expect("Failed to execute request.");

        assert_eq!(
            400,
            response.status().as_u16(),
            // Additional customised error message on test failure
            "The API did not fail with 400 Bad Request when the payload was {}.",
            error_message
        );
    }

}