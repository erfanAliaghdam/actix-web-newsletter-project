use actix_web::{web, Responder, HttpResponse};
use crate::extractors::SubscribeFormData;


pub async fn subscribe(_form: web::Form<SubscribeFormData>) -> impl Responder {
    let message = format!("user with email {} subscribed successfully!. ", _form.email);
    let response_body = serde_json::json!(
        {
            "status": "success",
            "message": message.to_string()
        }
    );
    HttpResponse::Ok()
        .content_type("application/json")
        .json(response_body)
}