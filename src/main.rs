use actix_web::{web, App, HttpRequest, HttpServer, Responder, HttpResponse, http};

async fn health_check(req: HttpRequest)-> impl Responder{
    let name = req.match_info().get("name").unwrap_or("World!");
    println!("Hello, {}!.", &name);
    let response = serde_json::json!(
        {
            "status": "success",
            "message": "server is working.... ."
        }
    );
    HttpResponse::build(http::StatusCode::OK).json(response)
}


#[tokio::main]
async fn main() -> std::io::Result<()> {
    println!("⚡ Server running at http://127.0.0.1:8000");

    HttpServer::new(|| { App::new()
        .route("/health-check", web::get().to(health_check))
        .route("/health-check/{name}", web::get().to(health_check))
    })
        .bind("127.0.0.1:8000")?
        .run()
        .await
}
