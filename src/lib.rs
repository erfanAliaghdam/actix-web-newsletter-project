use actix_web::{web, App, HttpServer};
mod handlers {
    pub mod health_check_handler;
}
use crate::handlers::health_check_handler::health_check;
use actix_web::dev::Server;

pub fn run() -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| { App::new()
        .route("/health-check", web::get().to(health_check))
        .route("/health-check/{name}", web::get().to(health_check))
    })
        .bind("127.0.0.1:8000")?
        .run();
    println!("⚡ Server running at http://127.0.0.1:8000");
    Ok(server)
}