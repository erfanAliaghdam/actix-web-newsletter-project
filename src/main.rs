use actix_web_newsletter_project::run;


#[tokio::main]
async fn main() -> std::io::Result<()> {
    run()?.await
}
