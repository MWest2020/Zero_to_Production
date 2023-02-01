use crate::routes::{health_check, subscribe};
use actix_web::dev::Server;
use actix_web::{web, App, HttpServer};
use std::net::TcpListener;

pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    //need a server start one.. with options first defined??
    let server = HttpServer::new(|| {
        //start app
        App::new()
            .route("/health_check", web::get().to(health_check))
            .route("/subscriptions", web::post().to(subscribe))
    })
    .listen(listener)? //error handling shorthand for the Result<()> enum
    .run(); // run the server
            // .await // interesting placement
    Ok(server)
}
