use actix_web::{web, App, HttpServer};
use actix_web::dev::Server;
use std::net::TcpListener;
use crate::handlers::{subscribe, health_check};


pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    let server_address =  listener.local_addr().unwrap();
    let server = HttpServer::new(|| { App::new()
        .route("/api/health-check", web::get().to(health_check))
        .route("/api/health-check/{name}", web::get().to(health_check))
        .route("/api/subscribe", web::post().to(subscribe))

    })
        .listen(listener)?
        .run();
    println!("⚡ Server running at http://{}", server_address);
    Ok(server)
}