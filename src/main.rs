use std::net::TcpListener;
use actix_web_newsletter_project::run;


#[tokio::main]
async fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8000")
        .expect("Failed to bind random port");
    run(listener)?.await
}
