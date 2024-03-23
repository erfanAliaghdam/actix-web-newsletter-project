use reqwest::StatusCode;
use actix_web_newsletter_project::run;

#[tokio::test]
async fn health_check_works() {
     spawn_app();
     let client = reqwest::Client::new();
     let response = client
         .get("http://127.0.0.1:8000/health-check")
         .send()
         .await
         .expect("Failed to execute request.");
     assert_eq!(response.status(), StatusCode::OK);

     let body = response
         .text()
         .await
         .expect("Failed to read response body as text.");

     // Parse the response body as JSON
     let json_body: serde_json::Value = serde_json::from_str(&body)
         .expect("Failed to parse response body as JSON.");

     assert_eq!(
          json_body,
          serde_json::json!({
            "status": "success",
            "message": "server is working.... ."
        })
     );
}

fn spawn_app() {
     let server = run().expect("Failed to bind address");
     // launch the server at background
     let _ = tokio::spawn(server);
}