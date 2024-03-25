use actix_web::{HttpRequest, Responder, HttpResponse};



pub async fn health_check(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("World!");
    println!("Hello, {}!.", &name);
    let response_body = serde_json::json!(
        {
            "status": "success",
            "message": "server is working.... ."
        }
    );
    HttpResponse::Ok()
        .content_type("application/json")
        .json(response_body)
}