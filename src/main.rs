use std::net::TcpListener;
use actix_web_newsletter_project::configuration::Settings;
use actix_web_newsletter_project::runner::run;


#[tokio::main]
async fn main() -> std::io::Result<()> {
    let configuration = Settings::load_from_env()
        .expect("Failed to load configuration.");
    let listener = TcpListener::bind(
        format!(
            "0.0.0.0:{}",
            configuration.app_port
        )
    ).expect("Failed to bind random port");
    run(listener)?.await
}
